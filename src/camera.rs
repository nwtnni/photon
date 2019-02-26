use crate::geometry::{Ray, Vec3};

pub struct Camera {
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, toward: Vec3, up: Vec3, fov: f32, aspect: f32) -> Self {
        let theta = fov * std::f32::consts::PI / 180.0;
        let height = (theta / 2.0).tan(); 
        let width = aspect * height;
        let w = (origin - toward).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);
        let corner = origin - u * width - v * height - w;
        let horizontal = u * 2.0 * width;
        let vertical = v * 2.0 * height;
        Camera { corner, horizontal, vertical, origin }
    }

    pub fn get(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
