use crate::geometry::Vec3;

pub trait Light {
    fn sample(&self) -> Vec3;
}
