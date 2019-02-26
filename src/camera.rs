use crate::geometry::{Ray, Vec3};

pub struct Camera {
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(fov: f32, aspect: f32) -> Self {
        let theta = fov * std::f32::consts::PI / 180.0;
        let height = (theta / 2.0).tan(); 
        let width = aspect * height;
        let corner = Vec3::new(-width, -height, -1.0);
        let horizontal = Vec3::new(2.0 * width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0 * height, 0.0);
        let origin = Vec3::default();
        Camera { corner, horizontal, vertical, origin }
    }

    pub fn get(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
