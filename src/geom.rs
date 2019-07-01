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
pub struct Record<'scene> {
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
    fn hit(&self, ray: &mut math::Ray, record: &mut Record<'scene>) -> bool;
    fn hit_any(&self, ray: &math::Ray) -> bool;
}

impl<'scene, T> Surface<'scene> for &T where T: Surface<'scene> + ?Sized {
    fn bound(&self) -> Box3 {
        (*self).bound()
    }
    fn hit(&self, ray: &mut math::Ray, record: &mut Record<'scene>) -> bool {
        (*self).hit(ray, record)
    }
    fn hit_any(&self, ray: &math::Ray) -> bool {
        (*self).hit_any(ray)
    }
}
