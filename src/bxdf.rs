use crate::math::Vec3;

mod lambertian;

pub use lambertian::Lambertian;

#[derive(Copy, Clone, Debug, Default)]
pub struct Record {
    n: Vec3,
    wi: Vec3,
    wo: Vec3,
    discrete: bool,
}

pub trait BXDF {
    fn eval(&self, wi: &Vec3, wo: &Vec3, n: &Vec3, out: &mut Vec3);
    fn sample(&self, sample: &mut Record, out: &mut Vec3) -> f32;
    fn pdf(&self, wi: &Vec3, wo: &Vec3, n: &Vec3) -> f32;
}
