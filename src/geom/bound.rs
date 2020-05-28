use crate::math::{Axis, Ray, Vec3};
use crate::geom;

#[readonly::make]
#[derive(Copy, Clone, Debug)]
pub struct Box3 {
    pub min: Vec3, 
    pub max: Vec3,
}

impl Box3 {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        let min = a.min(&b);
        let max = a.max(&b);
        Box3 { min, max }
    }

    pub fn max_extent(&self) -> Axis {
        let extent = (self.max - self.min).abs();
        let x = extent.x();
        let y = extent.y();
        let z = extent.z();
        if x > y {
            if x > z { Axis::X } else { Axis::Z }
        } else {
            if y > z { Axis::Y } else { Axis::Z }
        }
    }

    pub fn intersect(&self, rhs: &Self) -> Self {
        let min = self.min.max(&rhs.min);
        let max = self.max.min(&rhs.max);
        Box3 { min, max }
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

    pub fn scale(&self, c: f32) -> Self {
        let min = self.min * c;
        let max = self.max * c;
        Box3 { min, max }
    }

    pub fn translate(&self, v: &Vec3) -> Self {
        let min = self.min + v;
        let max = self.max + v;
        Box3 { min, max }
    }

    pub fn offset(&self, v: &Vec3) -> Vec3 {
        let m = self.max.gt(&self.min);
        let p = v - self.min;
        m.blend(&p, &(p / (self.max - self.min)))
    }

    pub fn surface_area(&self) -> f32 {
        2.0 * (self.max - self.min).len_sq()
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

    fn hit(&self, ray: &mut Ray, _: &mut geom::Hit<'scene>) -> bool {
        self.hit_any(&*ray)
    }

    /// See: https://medium.com/@bromanz/another-view-on-the-classic-ray-aabb-intersection-algorithm-for-bvh-traversal-41125138b525
    fn hit_any(&self, ray: &Ray) -> bool {
        if cfg!(feature = "stats") {
            crate::stats::INTERSECTION_TESTS.inc();
            crate::stats::BOUNDING_BOX_INTERSECTION_TESTS.inc();
        }

        let t_0 = (self.min - ray.p) * ray.inv;
        let t_1 = (self.max - ray.p) * ray.inv;

        let t_min = t_0.min(&t_1).max_horizontal();
        let t_max = t_0.max(&t_1).min_horizontal();

        t_min < t_max && t_min < ray.max && t_max > ray.min
    }
}
