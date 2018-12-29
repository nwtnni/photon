use noisy_float::prelude::*;

use crate::math::{Point2f, Ray};

mod orthographic;
mod perspective;

pub trait Camera {
    fn generate_ray(sample: &Sample) -> Ray;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Sample {
    film: Point2f,  
    lens: Point2f,
    time: N32,
}
