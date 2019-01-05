use noisy_float::prelude::*;

use crate::geometry::Intersection;
use crate::math::{Normal3f, Vec3f, Spectrum3f};

mod lambertian;

pub use self::lambertian::Lambertian;

pub struct BSDF<'scene> {
    eta: N32,
    normal: Normal3f,
    tan_u: Vec3f, 
    tan_v: Vec3f,
    bxdfs: Vec<&'scene BxDF>,
}

impl<'scene> BSDF<'scene> {
    pub fn new(intersection: Intersection, eta: N32, bxdfs: Vec<&'scene BxDF>) -> Self {
        let normal = intersection.n;
        let tan_u = intersection.dp_du.normalize();
        let tan_v = intersection.dp_dv.normalize();
        BSDF {
            eta,
            normal,
            tan_u,
            tan_v,
            bxdfs,
        }
    }

    pub fn world_to_local(&self, v: &Vec3f) -> Vec3f {
        Vec3f::new(
            v.dot_v(&self.tan_u),
            v.dot_v(&self.tan_v),
            v.dot_n(&self.normal),
        )
    }

    pub fn local_to_world(&self, v: &Vec3f) -> Vec3f {
        Vec3f::new(
            self.tan_u.x() * v.x() + self.tan_v.x() * v.y() + self.normal.x() * v.z(),
            self.tan_u.y() * v.x() + self.tan_v.y() * v.y() + self.normal.y() * v.z(),
            self.tan_u.z() * v.x() + self.tan_v.z() * v.y() + self.normal.z() * v.z(),
        )
    }

    pub fn eval(&self, wo_world: &Vec3f, wi_world: &Vec3f) -> Spectrum3f {
        let wi_local = self.world_to_local(wi_world);
        let wo_local = self.world_to_local(wo_world);
        let reflect = self.normal.dot_v(wo_world) * self.normal.dot_v(wi_world) > n32(0.0);
        let mut spectrum = Spectrum3f::default();
        for bxdf in &self.bxdfs {
            if reflect && bxdf.transfer().reflects()
            || !reflect && bxdf.transfer().transmits() {
                spectrum += bxdf.eval(wo_local, wi_local);
            }
        }
        spectrum
    }
}

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

impl Transfer {
    pub fn reflects(&self) -> bool {
        match self {
        | Transfer::Reflect
        | Transfer::Scatter => true,
        | Transfer::Transmit => false,
        }
    }

    pub fn transmits(&self) -> bool {
        match self {
        | Transfer::Reflect
        | Transfer::Transmit => true,
        | Transfer::Scatter => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Material {
    Diffuse,
    Glossy,
    Specular,
}
