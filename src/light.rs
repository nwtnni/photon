use std::fmt;

mod point;

pub use point::Point;

use crate::geom::Vec3;

pub trait Light: fmt::Debug + Send + Sync {
    fn sample(&self, point: &Vec3, wi: &mut Vec3) -> f32;
}
