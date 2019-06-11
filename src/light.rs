use std::fmt;

use crate::math;

mod point;
mod rect;

pub use point::Point;

#[readonly::make]
#[derive(Copy, Clone, Debug, Default)]
pub struct Sample {
    /// Direction to light
    pub d: math::Vec3,

    /// Distance to light
    pub t: f32,

    /// Attenuation
    pub a: f32,

    /// Probability
    pub p: f32,
}

pub trait Light: fmt::Debug + Send + Sync {
    fn eval(&self, ray: &math::Ray) -> math::Vec3;
    fn sample(&self, point: &math::Vec3) -> Sample;
    fn pdf(&self, ray: &math::Ray) -> f32;
    fn downcast_point(&self) -> Option<Point>;
}
