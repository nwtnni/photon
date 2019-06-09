use crate::math::{self, Axis, Ray, Vec3};
use crate::geom;

#[derive(Copy, Clone, Debug)]
pub struct Box3 {
    min: Vec3, 
    max: Vec3,
}

impl Box3 {
    #[inline(always)]
    pub fn new(a: Vec3, b: Vec3) -> Self {
        let min = a.min(&b);
        let max = a.max(&b);
        Box3 { min, max }
    }

    #[inline(always)]
    pub fn min(&self) -> Vec3 {
        self.min
    }

    #[inline(always)]
    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn max_extent(&self) -> Axis {
        let x = (self.max[0] - self.min[0]).abs();
        let y = (self.max[1] - self.min[1]).abs();
        let z = (self.max[2] - self.min[2]).abs();
        if x > y {
            if x > z { Axis::X } else { Axis::Z }
        } else {
            if y > z { Axis::Y } else { Axis::Z }
        }
    }

    pub fn union_b(&self, rhs: &Self) -> Self {
        let min = self.min.min(&rhs.min);
        let max = self.max.max(&rhs.max);
        Box3 { min, max }
    }

    pub fn union_v(&self, rhs: &Vec3) -> Self {
        let min = self.min.min(rhs);
        let max = self.max.max(rhs);
        Box3 { min, max }
    }

    pub fn smallest() -> Self {
        let min = std::f32::NEG_INFINITY;
        let max = std::f32::INFINITY;
        Box3 {
            min: Vec3::new(max, max, max),
            max: Vec3::new(min, min, min),
        }
    }

    pub fn offset(&self, v: &Vec3) -> Vec3 {
        let mut o = v - self.min;
        if self.max[0] > self.min[0] { o[0] /= self.max[0] - self.min[0] }
        if self.max[1] > self.min[1] { o[1] /= self.max[1] - self.min[1] }
        if self.max[2] > self.min[2] { o[2] /= self.max[2] - self.min[2] }
        o
    }

    pub fn surface_area(&self) -> f32 {
        let d = self.max - self.min;
        2.0 * (d.x() * d.x() + d.y() * d.y() + d.z() * d.z())
    }
}

impl Default for Box3 {
    fn default() -> Self {
        Box3::smallest()
    }
}

impl std::ops::Index<usize> for Box3 {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
        | 0 => &self.min,
        | 1 => &self.max,
        | _ => unreachable!(),
        }
    }
}

impl<'scene> geom::Surface<'scene> for Box3 {
    fn bound(&self) -> Box3 {
        *self
    }

    fn hit(&self, ray: &mut Ray, _: &mut geom::Record<'scene>) -> bool {
        self.hit_any(&*ray)
    }

    /// See [PBRT][0], the [bvh][1] crate, and the [original paper][2].
    ///
    /// [0]: https://github.com/mmp/pbrt-v3/blob/3e9dfd72c6a669848616a18c22f347c0810a0b51/src/core/geometry.h#L1411-L1438
    /// [1]: https://github.com/svenstaro/bvh/blob/15ca10d07e40036135621cd80df2e5ba024a5991/src/ray.rs#L85-L150
    /// [2]: http://www.cs.utah.edu/~awilliam/box/box.pdf 
    fn hit_any(&self, ray: &Ray) -> bool {
        if cfg!(feature = "stats") {
            crate::stats::INTERSECTION_TESTS.inc();
            crate::stats::BOUNDING_BOX_INTERSECTION_TESTS.inc();
        }

        let x_min = (self[ray.sign[0]].x() - ray.o.x()) * ray.inv[0];
        let x_max = (self[1 - ray.sign[0]].x() - ray.o.x()) * ray.inv[0];

        let y_min = (self[ray.sign[1]].y() - ray.o.y()) * ray.inv[1];
        let y_max = (self[1 - ray.sign[1]].y() - ray.o.y()) * ray.inv[1];

        if x_min > y_max || y_min > x_max { return false }

        let ray_min = math::max(x_min, y_min);
        let ray_max = math::min(x_max, y_max);

        let z_min = (self[ray.sign[2]].z() - ray.o.z()) * ray.inv[2];
        let z_max = (self[1 - ray.sign[2]].z() - ray.o.z()) * ray.inv[2];

        if ray_min > z_max || z_min > ray_max { return false }

        let ray_min = math::max(ray_min, z_min);
        let ray_max = math::min(ray_max, z_max);

        ray_min < ray.max && ray_max > ray.min
    }
}
