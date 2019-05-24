use crate::geometry::{Axis, Ray, Vec3};
use crate::surface::{Hit, Surface};

#[derive(Copy, Clone, Debug)]
pub struct Bound {
    min: Vec3, 
    max: Vec3,
}

impl Bound {
    #[inline(always)]
    pub fn new(a: Vec3, b: Vec3) -> Self {
        let min = a.min(&b);
        let max = a.max(&b);
        Bound { min, max }
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
        Bound { min, max }
    }

    pub fn union_v(&self, rhs: &Vec3) -> Self {
        let min = self.min.min(rhs);
        let max = self.max.max(rhs);
        Bound { min, max }
    }

    pub fn smallest() -> Self {
        let min = std::f32::MIN;
        let max = std::f32::MAX;
        Bound {
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

    pub fn hit_inv(&self, ray: &Ray, inv: &[f32; 3]) -> bool {
        if cfg!(feature = "stats") {
            crate::stats::INTERSECTION_TESTS.inc();
            crate::stats::BOUNDING_BOX_INTERSECTION_TESTS.inc();
        }
        for i in 0..3 {
            let pos = ray.o[i];
            let inv = inv[i];
            let mut t0 = (self.min[i] - pos) * inv;
            let mut t1 = (self.max[i] - pos) * inv;
            if inv < 0.0 { std::mem::swap(&mut t0, &mut t1) }
            let t_min = if t0 > ray.min { t0 } else { ray.min };
            let t_max = if t1 < ray.max { t1 } else { ray.max };
            if t_max <= t_min { return false }
        }
        true
    }
}

impl Default for Bound {
    fn default() -> Self {
        let min = std::f32::MIN;
        let max = std::f32::MAX;
        Bound {
            min: Vec3::new(min, min, min),
            max: Vec3::new(max, max, max),
        }
    }

}

impl<'scene> Surface<'scene> for Bound {
    fn bound(&self, _: f32, _: f32) -> Bound {
        *self
    }

    fn hit(&self, ray: &mut Ray, _: &mut Hit<'scene>) -> bool {
        self.hit_any(&*ray)
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        if cfg!(feature = "stats") {
            crate::stats::INTERSECTION_TESTS.inc();
            crate::stats::BOUNDING_BOX_INTERSECTION_TESTS.inc();
        }
        for i in 0..3 {
            let pos = ray.o[i];
            let inv = 1.0 / ray.d[i];
            let mut t0 = (self.min[i] - pos) * inv;
            let mut t1 = (self.max[i] - pos) * inv;
            if inv < 0.0 { std::mem::swap(&mut t0, &mut t1) }
            let t_min = if t0 > ray.min { t0 } else { ray.min };
            let t_max = if t1 < ray.max { t1 } else { ray.max };
            if t_max <= t_min { return false }
        }
        true
    }
}
