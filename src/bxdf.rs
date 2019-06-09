use crate::math::Vec3;

mod lambertian;

pub use lambertian::Lambertian;

#[derive(Copy, Clone, Debug, Default)]
pub struct Record {
    w: Vec3,
    bxdf: Vec3,  
    discrete: bool,
    probability: f32, 
}

pub trait BxDF: std::fmt::Debug + Send + Sync {
    fn eval(&self, wi: &Vec3, wr: &Vec3, n: &Vec3) -> Vec3;
    fn sample(&self, w: &Vec3, n: &Vec3, sample: &mut Record);
    fn pdf(&self, wi: &Vec3, wr: &Vec3, n: &Vec3) -> f32;
}
