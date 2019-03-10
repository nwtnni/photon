use crate::geometry::Vec3;
use crate::light::Light;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    position: Vec3,
    intensity: Vec3,
}

impl Point {
    pub fn new(position: Vec3, intensity: Vec3) -> Self {
        Point { position, intensity }
    }
}

impl Light for Point {
    fn sample(&self, point: &Vec3, wi: &mut Vec3) -> f32 {
        *wi = (self.position - point).normalize();
        1.0
    }
}
