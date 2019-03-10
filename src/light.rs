mod point;

pub use point::Point;

use crate::geometry::Vec3;

pub trait Light {
    fn sample(&self, point: &Vec3, wi: &mut Vec3) -> f32;
}
