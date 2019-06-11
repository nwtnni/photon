use std::fmt;

use crate::math;

mod point;
mod rect;

pub use point::Point;

#[derive(Copy, Clone, Debug, Default)]
pub struct Record {
    /// Position on light
    pub l: math::Vec3,

    /// Attenuation
    pub a: f32,

    /// Probability
    pub p: f32,
}

pub trait Light: fmt::Debug + Send + Sync {
    fn eval(&self, ray: &math::Ray) -> math::Vec3;
    fn sample(&self, point: &math::Vec3, record: &mut Record);
    fn pdf(&self, ray: &math::Ray) -> f32;
    fn downcast_point(&self) -> Option<Point>;
}
