use crate::geom::{Ray, Vec3};
use crate::surface::Hit;

mod diffuse;
mod dielectric;
mod metal;
mod normal;

pub use diffuse::Diffuse;
pub use dielectric::Dielectric;
pub use metal::Metal;
pub use normal::Normal;

/// Represents a material that can scatter light.
pub trait Material<'scene>: std::fmt::Debug + Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &Hit<'scene>, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

/// Reflects vector `v` over normal vector `n`.
pub(crate) fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(&n) * 2.0
}

/// Refracts vector `v` at a surface with normal vector `n` and
/// refractive index ratio `ni_over_nt`.
pub(crate) fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(((uv - n * dt) * ni_over_nt) - n * discriminant.sqrt())
    } else {
        None
    }
}
