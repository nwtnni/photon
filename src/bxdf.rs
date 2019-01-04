use crate::math::{Vec3f, Spectrum3f};

mod lambertian;

pub use self::lambertian::Lambertian;

pub trait BxDF {
    fn eval(&self, wo: Vec3f, wi: Vec3f) -> Spectrum3f;
    fn transfer(&self) -> Transfer;
    fn material(&self) -> Material;
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Transfer {
    Reflect,
    Transmit,
    Scatter,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Material {
    Diffuse,
    Glossy,
    Specular,
}
