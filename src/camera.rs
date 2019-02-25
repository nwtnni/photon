use crate::geometry::{Ray, Vec3};

pub struct Camera {
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(corner: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Self {
        Camera { corner, horizontal, vertical, origin }
    }

    pub fn get(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
