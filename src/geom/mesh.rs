use crate::arena;
use crate::bvh;
use crate::bxdf;
use crate::geom;
use crate::math;

#[derive(Copy, Clone, Debug)]
pub struct Mesh<'scene> {
    bxdf: &'scene dyn bxdf::BxDF,
    internal: bvh::Linear<'scene, geom::Tri<'scene>>,
}

impl<'scene> Mesh<'scene> {
    pub fn new(
        arena: &'scene arena::Arena,
        bxdf: &'scene dyn bxdf::BxDF,
        triangles: &[geom::Tri<'scene>],
    ) -> Self {
        let internal = bvh::Linear::new(arena, triangles);
        Mesh { bxdf, internal }
    }
}

impl<'scene> geom::Surface<'scene> for Mesh<'scene> {
    fn bound(&self) -> geom::Box3 {
        self.internal.bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Hit<'scene>) -> bool {
        if self.internal.hit(ray, hit) {
            hit.bxdf = Some(self.bxdf);
            hit.emit = None;
            true
        } else {
            false
        }
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.internal.hit_any(ray)
    }
}
