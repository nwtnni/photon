use crate::math;
use crate::light;

#[readonly::make]
#[derive(Copy, Clone, Debug)]
pub struct Point {
    /// Position
    pub p: math::Vec3,

    /// Intensity
    pub i: math::Vec3,
}

impl Point {
    pub fn new(position: math::Vec3, intensity: math::Vec3) -> Self {
        Point {
            p: position,
            i: intensity,
        }
    }
}

impl light::Light for Point {
    fn intensity(&self) -> math::Vec3 {
        self.i    
    }

    fn sample(&self, p: &math::Vec3, r: &mut light::Record) {
        let wi = self.p - p;
        r.d = wi.normalize();
        r.a = 1.0 / wi.len_sq();
        r.t = wi.len();
        r.p = 1.0;
    }

    fn pdf(&self, _: &math::Ray) -> f32 {
        1.0
    }

    fn downcast_point(&self) -> Option<Point> {
        Some(*self)
    }
}
