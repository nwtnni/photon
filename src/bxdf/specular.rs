use crate::math;

#[derive(Copy, Clone, Debug)]
pub struct Specular {
    color: math::Vec3,
    eta: f32,
}
