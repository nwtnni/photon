use crate::math;

mod lambertian;
mod specular;

pub use lambertian::Lambertian;
pub use specular::Specular;

#[derive(Copy, Clone, Debug, Default)]
pub struct Record {
    w: math::Vec3,
    bxdf: math::Vec3,  
    discrete: bool,
    probability: f32, 
}

pub trait BxDF: std::fmt::Debug + Send + Sync {
    fn eval(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> math::Vec3;
    fn sample(&self, w: &math::Vec3, n: &math::Vec3, sample: &mut Record);
    fn pdf(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> f32;
}

pub fn dieletric(v: math::Vec3, n: math::Vec3, eta: f32) -> f32 {
    let cos_i = v.dot(&n);
    if cos_i < 0.0 { return 0.0 }

    let cos_t = 1.0 - (1.0 - cos_i.powi(2)) / eta.powi(2);
    if cos_t < 0.0 { return 1.0 }

    let parallel = (eta * cos_i - cos_t) / (eta * cos_i + cos_t);
    let perpendicular = (cos_i - eta * cos_t) / (cos_i + eta * cos_t);

    0.5 * (parallel.powi(2) + perpendicular.powi(2))
}
