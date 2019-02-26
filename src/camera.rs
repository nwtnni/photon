use crate::geometry::{Ray, Vec3};

pub struct Camera {
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens: f32,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        toward: Vec3,
        up: Vec3,
        fov: f32,
        aspect: f32,
        aperture: f32,
        focus: f32,
    ) -> Self {
        let theta = fov * std::f32::consts::PI / 180.0;
        let height = (theta / 2.0).tan();
        let width = aspect * height;
        let w = (origin - toward).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);
        let corner = origin - u * focus * width - v * focus * height - w * focus;
        let horizontal = u * 2.0 * focus * width;
        let vertical = v * 2.0 * focus * height;
        Camera {
            corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            lens: aperture / 2.0,
        }
    }

    fn random_offset(&self) -> Vec3 {
        let ones = Vec3::new(1.0, 1.0, 0.0);
        let d = loop {
            let d = Vec3::new(rand::random(), rand::random(), 0.0) * 2.0 - ones;
            if d.len_sq() < 1.0 { break d }
        };
        self.u * self.lens * d.x() +
        self.v * self.lens * d.y()
    }

    pub fn get(&self, u: f32, v: f32) -> Ray {
        let offset = self.random_offset();
        Ray::new(
            self.origin + offset,
            self.corner + self.horizontal * u
                        + self.vertical * v
                        - self.origin
                        - offset,
        )
    }
}
