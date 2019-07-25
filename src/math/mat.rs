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
        Mat4([
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn translate_y(y: f32) -> Self {
        Mat4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn translate_z(z: f32) -> Self {
        Mat4([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }
}
