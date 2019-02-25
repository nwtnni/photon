use crate::geometry::{Ray, Vec3};
use crate::surface::Hit;

mod diffuse;
mod metal;

pub use diffuse::Diffuse;
pub use metal::Metal;

pub trait Material: std::fmt::Debug {
    fn scatter<'scene>(&self, ray: &Ray, hit: &'scene Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}
