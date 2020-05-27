use crate::math;
use crate::bxdf;

mod bound;
mod sphere;
mod mesh;
mod tri;
mod translate;
mod quad;
mod sdf;

pub use bound::Box3;
pub use mesh::Mesh;
pub use sdf::{SDF, Shape};
pub use sphere::Sphere;
pub use quad::Quad;
pub use translate::Translate;
pub use tri::Tri;

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
    pub p: math::Vec3,

    /// Normal at hit point
    pub n: math::Vec3,

    /// BxDF at hit point
    pub bxdf: Option<&'scene dyn bxdf::BxDF>,

    /// Light emission at hit point
    pub emit: Option<math::Vec3>,
}

/// Represents an object that can interact with light rays.
pub trait Surface<'scene>: std::fmt::Debug + Send + Sync {
    fn bound(&self) -> Box3;
    fn hit(&self, ray: &mut math::Ray, hit: &mut Hit<'scene>) -> bool;
    fn hit_any(&self, ray: &math::Ray) -> bool;
}

impl<'scene, T> Surface<'scene> for &T where T: Surface<'scene> + ?Sized {
    fn bound(&self) -> Box3 {
        (*self).bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut Hit<'scene>) -> bool {
        (*self).hit(ray, hit)
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        (*self).hit_any(ray)
    }
}

/// Represents an object that can interact with light rays.
#[derive(Copy, Clone, Debug)]
pub enum Any<'scene> {
    Box3(Box3),
    Mesh(Mesh<'scene>),
    Sphere(Sphere<'scene>),
    Quad(Quad<'scene>),
    Translate(Translate<'scene>),
    Tri(Tri<'scene>),
}

impl<'scene> Surface<'scene> for Any<'scene> {
    fn bound(&self) -> Box3 {
        match self {
            Any::Box3(surface) => surface.bound(),
            Any::Mesh(surface) => surface.bound(),
            Any::Sphere(surface) => surface.bound(),
            Any::Quad(surface) => surface.bound(),
            Any::Translate(surface) => surface.bound(),
            Any::Tri(surface) => surface.bound(),
        }
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut Hit<'scene>) -> bool {
        match self {
            Any::Box3(surface) => surface.hit(ray, hit),
            Any::Mesh(surface) => surface.hit(ray, hit),
            Any::Sphere(surface) => surface.hit(ray, hit),
            Any::Quad(surface) => surface.hit(ray, hit),
            Any::Translate(surface) => surface.hit(ray, hit),
            Any::Tri(surface) => surface.hit(ray, hit),
        }
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        match self {
            Any::Box3(surface) => surface.hit_any(ray),
            Any::Mesh(surface) => surface.hit_any(ray),
            Any::Sphere(surface) => surface.hit_any(ray),
            Any::Quad(surface) => surface.hit_any(ray),
            Any::Translate(surface) => surface.hit_any(ray),
            Any::Tri(surface) => surface.hit_any(ray),
        }
    }
}
