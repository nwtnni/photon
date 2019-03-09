use crate::geometry::{Bound, Ray, Vec3};
use crate::material::Material;

mod sphere;
mod list;

pub use list::List;
pub use sphere::Sphere;

/// Carries information about surface intersections.
#[derive(Copy, Clone, Debug, Default)]
pub struct Hit<'scene> {
    /// Hit time
    pub t: f32,

    /// Texture coordinate
    pub u: f32,

    /// Texture coordinate
    pub v: f32,

    /// Hit point
    pub p: Vec3,

    /// Normal at hit point
    pub n: Vec3,

    /// Material at hit point
    pub m: Option<&'scene dyn Material<'scene>>,
}

/// Represents an object that can interact with light rays.
pub trait Surface<'scene>: std::fmt::Debug + Send + Sync {
    fn bound(&self, t0: f32, t1: f32) -> Bound;
    fn hit(&self, ray: &mut Ray, record: &mut Hit<'scene>) -> bool;
    fn hit_any(&self, ray: &Ray) -> bool;
}
