use crate::math;

#[derive(Copy, Clone, Debug)]
pub struct Mat4([f32; 16]);

impl Mat4 {
    pub fn translate(v: math::Vec3) -> Self {
        Mat4([
            1.0, 0.0, 0.0, v.x(),
            0.0, 1.0, 0.0, v.y(),
            0.0, 0.0, 1.0, v.z(),
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn translate_x(x: f32) -> Self {
        Self::translate(math::Vec3::new(x, 0.0, 0.0))
    }

    pub fn translate_y(y: f32) -> Self {
        Self::translate(math::Vec3::new(0.0, y, 0.0))
    }

    pub fn translate_z(z: f32) -> Self {
        Self::translate(math::Vec3::new(0.0, 0.0, z))
    }

    pub fn scale(s: math::Vec3) -> Self {
        Mat4([
            s.x(), 0.0, 0.0, 0.0,
            0.0, s.y(), 0.0, 0.0,
            0.0, 0.0, s.z(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn scale_x(s: f32) -> Self {
        Self::scale(math::Vec3::new(s, 0.0, 0.0))
    }

    pub fn scale_y(s: f32) -> Self {
        Self::scale(math::Vec3::new(0.0, s, 0.0))
    }

    pub fn scale_z(s: f32) -> Self {
        Self::scale(math::Vec3::new(0.0, 0.0, s))
    }
}
