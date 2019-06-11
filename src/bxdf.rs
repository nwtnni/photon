use std::mem;

use crate::math;

mod lambertian;

pub use lambertian::Lambertian;

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

pub fn dieletric(mut cos_theta_i: f32, mut eta_i: f32, mut eta_t: f32) -> f32 {

    cos_theta_i = math::clamp(cos_theta_i, -1.0, 1.0);

    if cos_theta_i <= 0.0 {
        mem::swap(&mut eta_i, &mut eta_t);
        cos_theta_i = cos_theta_i.abs();
    }

    let sin_theta_i = math::max(0.0, 1.0 - cos_theta_i.powi(2)).sqrt();
    let sin_theta_t = sin_theta_i * eta_i / eta_t;

    if sin_theta_t >= 1.0 { return 1.0 }

    let cos_theta_t = math::max(0.0, 1.0 - sin_theta_t.powi(2)).sqrt();

    let parallel =
        ((eta_t * cos_theta_i) - (eta_i * cos_theta_t)) /
        ((eta_t * cos_theta_i) + (eta_i * cos_theta_t));

    let perpendicular =
        ((eta_i * cos_theta_i) - (eta_t * cos_theta_t)) /
        ((eta_i * cos_theta_i) + (eta_t * cos_theta_t));

    (parallel.powi(2) + perpendicular.powi(2)) / 2.0
}
