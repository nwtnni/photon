use crate::geometry::{Ray, Vec3};
use crate::surface::{Hit, Surface};

#[derive(Copy, Clone, Debug)]
pub struct Bound {
    min: Vec3, 
    max: Vec3,
}

impl Bound {
    #[inline(always)]
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Bound { min, max }
    }

    pub fn union(&self, rhs: &Self) -> Self {
        let min = self.min.min(&rhs.min);
        let max = self.max.max(&rhs.max);
        Bound { min, max }
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

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, _: &mut Hit<'scene>) -> bool {
        let o = ray.o();
        let d = ray.d();
        for i in 0..3 {
            let inv_d = 1.0 / d[i];
            let mut t0 = (self.min[i] - o[i]) * inv_d;
            let mut t1 = (self.max[i] - o[i]) * inv_d;
            if inv_d < 0.0 { std::mem::swap(&mut t0, &mut t1) }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min { return false }
        }
        true
    }
}
