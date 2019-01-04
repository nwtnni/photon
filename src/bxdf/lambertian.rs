use noisy_float::prelude::*;

use crate::bxdf::{BxDF, Transfer, Material};
use crate::math::{Spectrum3f, Vec3f};

pub struct Lambertian {
    reflectance: Spectrum3f,
}

impl BxDF for Lambertian {
    fn eval(&self, _: Vec3f, _: Vec3f) -> Spectrum3f {
        self.reflectance * n32(1.0 / std::f32::consts::PI)
    }

    fn transfer(&self) -> Transfer {
        Transfer::Reflect
    }

    fn material(&self) -> Material {
        Material::Diffuse
    }
}
