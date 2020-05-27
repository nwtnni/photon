use std::fmt;

use crate::geom;
use crate::math;

mod point;
mod quad;

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

impl<'a, L> Light for &'a L where L: Light + ?Sized {
    fn eval(&self, ray: &math::Ray) -> math::Vec3 {
        (*self).eval(ray)
    }

    fn sample(&self, point: &math::Vec3) -> Sample {
        (*self).sample(point)
    }

    fn pdf(&self, ray: &math::Ray) -> f32 {
        (*self).pdf(ray)
    }

    fn downcast_point(&self) -> Option<Point> {
        (*self).downcast_point()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Any<'scene> {
    Point(Point),
    Quad(geom::Quad<'scene>),
}

impl<'scene> Light for Any<'scene> {
    fn eval(&self, ray: &math::Ray) -> math::Vec3 {
        match self {
            Any::Point(light) => light.eval(ray),
            Any::Quad(light) => light.eval(ray),
        }
    }

    fn sample(&self, point: &math::Vec3) -> Sample {
        match self {
            Any::Point(light) => light.sample(point),
            Any::Quad(light) => light.sample(point),
        }
    }

    fn pdf(&self, ray: &math::Ray) -> f32 {
        match self {
            Any::Point(light) => light.pdf(ray),
            Any::Quad(light) => light.pdf(ray),
        }
    }

    fn downcast_point(&self) -> Option<Point> {
        match self {
            Any::Point(light) => light.downcast_point(),
            Any::Quad(light) => light.downcast_point(),
        }
    }
}
