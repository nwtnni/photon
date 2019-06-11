use std::fmt;

use crate::math;

mod point;
mod rect;

pub use point::Point;

pub struct Record {
    /// Direction
    pub d: math::Vec3,

    /// Attenuation
    pub a: f32,

    /// Distance along direction vector
    pub t: f32,

    /// Probability
    pub p: f32,
}

pub trait Light: fmt::Debug + Send + Sync {
    fn intensity(&self) -> math::Vec3;
    fn sample(&self, point: &math::Vec3, record: &mut Record);
    fn pdf(&self, ray: &math::Ray) -> f32;
    fn downcast_point(&self) -> Option<Point>;
}
