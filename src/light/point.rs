use crate::math::Vec3;
use crate::light::Light;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    /// Position
    p: Vec3,

    /// Intensity
    i: Vec3,
}

impl Point {
    pub fn new(position: Vec3, intensity: Vec3) -> Self {
        Point {
            p: position,
            i: intensity,
        }
    }
}

impl Light for Point {
    fn sample(&self, p: &Vec3, wi: &mut Vec3) -> f32 {
        *wi = (self.p - p).normalize();
        1.0
    }
}
