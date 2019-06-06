use crate::math::{Ray, Vec3, uniform_disk};

/// Source of light rays.
#[derive(Copy, Clone, Debug)]
pub struct Camera {
    /// Lower left corner
    corner: Vec3,

    /// Horizontal axis with depth of field
    horizontal: Vec3,

    /// Vertical axis with depth of field
    vertical: Vec3,

    /// Camera position
    origin: Vec3,

    /// Normalized horizontal axis
    u: Vec3,

    /// Normalized vertical axis
    v: Vec3,

    /// Lens radius
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

    /// Generate a random point within this camera's lens disk
    fn random_offset(&self) -> Vec3 {
        let d = uniform_disk();
        self.u * self.lens * d.x() +
        self.v * self.lens * d.y()
    }

    /// Generate a ray through normalized screen coordinates `(u, v)`,
    /// where both `u` and `v` are in the range `[0.0, 1.0]`.
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
