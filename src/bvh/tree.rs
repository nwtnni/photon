use std::arch::x86_64;

use crate::bvh;
use crate::math;
use crate::geom;

#[derive(Copy, Clone, Debug)]
pub struct Tree<'scene, S>(pub &'scene [Node<S>]);

impl<'scene, S> geom::Surface<'scene> for Tree<'scene, S> where S: geom::Surface<'scene> {
    fn bound(&self) -> geom::Box3 {
        self.0[0].bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Hit<'scene>) -> bool {
        let mut this = 0;
        let mut visit = Vec::with_capacity(32);
        let mut success = false;

        unsafe {
            let ray_px8 = x86_64::_mm256_set1_ps(ray.p.x());
            let ray_py8 = x86_64::_mm256_set1_ps(ray.p.y());
            let ray_pz8 = x86_64::_mm256_set1_ps(ray.p.z());

            let ray_ix8 = x86_64::_mm256_set1_ps(ray.inv.x());
            let ray_iy8 = x86_64::_mm256_set1_ps(ray.inv.y());
            let ray_iz8 = x86_64::_mm256_set1_ps(ray.inv.z());

            let ray_min = x86_64::_mm256_set1_ps(ray.min);
            let ray_max = x86_64::_mm256_set1_ps(ray.max);

            loop {
                match &self.0[this] {
                | Node::Leaf(leaf) => success |= leaf.hit(ray, hit),
                | Node::Node { children, bounds, .. } => {
                    let t0x = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.min_x, ray_px8), ray_ix8);
                    let t0y = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.min_y, ray_py8), ray_iy8);
                    let t0z = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.min_z, ray_pz8), ray_iz8);

                    let t1x = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.max_x, ray_px8), ray_ix8);
                    let t1y = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.max_y, ray_py8), ray_iy8);
                    let t1z = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.max_z, ray_pz8), ray_iz8);

                    let t_min_x = x86_64::_mm256_min_ps(t0x, t1x);
                    let t_min_y = x86_64::_mm256_min_ps(t0y, t1y);
                    let t_min_z = x86_64::_mm256_min_ps(t0z, t1z);
                    let t_min = x86_64::_mm256_max_ps(x86_64::_mm256_max_ps(t_min_x, t_min_y), t_min_z);

                    let t_max_x = x86_64::_mm256_max_ps(t0x, t1x);
                    let t_max_y = x86_64::_mm256_max_ps(t0y, t1y);
                    let t_max_z = x86_64::_mm256_max_ps(t0z, t1z);
                    let t_max = x86_64::_mm256_min_ps(x86_64::_mm256_min_ps(t_max_x, t_max_y), t_max_z);

                    let valid = x86_64::_mm256_movemask_ps(
                        x86_64::_mm256_and_ps(
                            x86_64::_mm256_and_ps(
                                x86_64::_mm256_cmp_ps(t_min, t_max, x86_64::_CMP_LT_OQ),
                                x86_64::_mm256_cmp_ps(t_min, ray_max, x86_64::_CMP_LT_OQ),
                            ),
                            x86_64::_mm256_cmp_ps(t_max, ray_min, x86_64::_CMP_GT_OQ),
                        )
                    );

                    for (index, child) in children.iter().enumerate() {
                        if (valid & (1 << index)) > 0 {
                            #[cfg(feature = "stats")]
                            crate::stats::BVH_HITS.inc();
                            visit.push(*child as usize);
                        } else if cfg!(feature = "stats") {
                            crate::stats::BVH_MISSES.inc()
                        }
                    }
                }
                }

                match visit.pop() {
                | None => break success,
                | Some(next) => this = next,
                }
            }
        }
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        let mut this = 0;
        let mut visit = Vec::with_capacity(32);

        unsafe {
            let ray_px8 = x86_64::_mm256_set1_ps(ray.p.x());
            let ray_py8 = x86_64::_mm256_set1_ps(ray.p.y());
            let ray_pz8 = x86_64::_mm256_set1_ps(ray.p.z());

            let ray_ix8 = x86_64::_mm256_set1_ps(ray.inv.x());
            let ray_iy8 = x86_64::_mm256_set1_ps(ray.inv.y());
            let ray_iz8 = x86_64::_mm256_set1_ps(ray.inv.z());

            let ray_min = x86_64::_mm256_set1_ps(ray.min);
            let ray_max = x86_64::_mm256_set1_ps(ray.max);

            loop {
                match &self.0[this] {
                | Node::Leaf(leaf) if leaf.hit_any(ray) => return true,
                | Node::Leaf(_) => (),
                | Node::Node { children, bounds, .. } => {
                    let t0x = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.min_x, ray_px8), ray_ix8);
                    let t0y = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.min_y, ray_py8), ray_iy8);
                    let t0z = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.min_z, ray_pz8), ray_iz8);

                    let t1x = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.max_x, ray_px8), ray_ix8);
                    let t1y = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.max_y, ray_py8), ray_iy8);
                    let t1z = x86_64::_mm256_mul_ps(x86_64::_mm256_sub_ps(bounds.max_z, ray_pz8), ray_iz8);

                    let t_min_x = x86_64::_mm256_min_ps(t0x, t1x);
                    let t_min_y = x86_64::_mm256_min_ps(t0y, t1y);
                    let t_min_z = x86_64::_mm256_min_ps(t0z, t1z);
                    let t_min = x86_64::_mm256_max_ps(x86_64::_mm256_max_ps(t_min_x, t_min_y), t_min_z);

                    let t_max_x = x86_64::_mm256_max_ps(t0x, t1x);
                    let t_max_y = x86_64::_mm256_max_ps(t0y, t1y);
                    let t_max_z = x86_64::_mm256_max_ps(t0z, t1z);
                    let t_max = x86_64::_mm256_min_ps(x86_64::_mm256_min_ps(t_max_x, t_max_y), t_max_z);

                    let valid = x86_64::_mm256_movemask_ps(
                        x86_64::_mm256_and_ps(
                            x86_64::_mm256_and_ps(
                                x86_64::_mm256_cmp_ps(t_min, t_max, x86_64::_CMP_LT_OQ),
                                x86_64::_mm256_cmp_ps(t_min, ray_max, x86_64::_CMP_LT_OQ),
                            ),
                            x86_64::_mm256_cmp_ps(t_max, ray_min, x86_64::_CMP_GT_OQ),
                        )
                    );

                    for (index, child) in children.iter().enumerate() {
                        if (valid & (1 << index)) > 0 {
                            #[cfg(feature = "stats")]
                            crate::stats::BVH_HITS.inc();
                            visit.push(*child as usize);
                        } else if cfg!(feature = "stats") {
                            crate::stats::BVH_MISSES.inc()
                        }
                    }
                }
                }

                match visit.pop() {
                | None => return false,
                | Some(next) => this = next,
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Node<S> {
    Leaf(bvh::Leaf<S>),
    Node {
        bound: geom::Box3,
        bounds: Box3SoA,
        children: [u32; 8],
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Box3SoA {
    pub min_x: x86_64::__m256,
    pub min_y: x86_64::__m256,
    pub min_z: x86_64::__m256,

    pub max_x: x86_64::__m256,
    pub max_y: x86_64::__m256,
    pub max_z: x86_64::__m256,
}

impl Default for Box3SoA {
    fn default() -> Self {
        unsafe {
            Box3SoA {
                min_x: x86_64::_mm256_setzero_ps(),
                min_y: x86_64::_mm256_setzero_ps(),
                min_z: x86_64::_mm256_setzero_ps(),
                max_x: x86_64::_mm256_setzero_ps(),
                max_y: x86_64::_mm256_setzero_ps(),
                max_z: x86_64::_mm256_setzero_ps(),
            }
        }
    }
}

impl<'scene, S> Node<S> where S: geom::Surface<'scene> {
    pub fn bound(&self) -> geom::Box3 {
        use geom::Surface;
        match self {
        | Node::Leaf(leaf) => leaf.bound(),
        | Node::Node { bound, .. } => *bound,
        }
    }
}
