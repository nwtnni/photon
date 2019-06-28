use crate::math;

mod lambertian;
mod specular;

pub use lambertian::Lambertian;
pub use specular::Specular;

#[readonly::make]
#[derive(Copy, Clone, Debug, Default)]
pub struct Sample {
    /// Direction
    pub d: math::Vec3,

    /// Value
    pub v: math::Vec3,  

    /// Probability
    pub p: f32, 

    /// Whether this sample came from a delta distribution
    pub delta: bool,
}

pub trait BxDF: std::fmt::Debug + Send + Sync {
    fn eval(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> math::Vec3;
    fn sample(&self, d: &math::Vec3, n: &math::Vec3) -> Sample;
    fn pdf(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> f32;
}
