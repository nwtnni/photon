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

pub fn dieletric(cos_i: f32, eta: f32) -> (f32, f32) {
    debug_assert!(cos_i >= 0.0);

    let cos_t = (1.0 - (1.0 - cos_i.powi(2)) / eta.powi(2)).sqrt();
    if cos_t < 0.0 { return (1.0, cos_t) }

    let parallel = (eta * cos_i - cos_t) / (eta * cos_i + cos_t);
    let perpendicular = (cos_i - eta * cos_t) / (cos_i + eta * cos_t);
    let reflect = (parallel.powi(2) + perpendicular.powi(2)) * 0.5;

    (reflect, cos_t)
}
