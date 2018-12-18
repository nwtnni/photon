mod bounds;
mod intersection;

pub use self::bounds::*;
pub use self::intersection::*;

use noisy_float::prelude::*;

use crate::math::{Ray, Transform};

/// Represents a geometric shape that can be intersected with a ray.
pub trait Shape {

    /// Bounding box in world space.
    fn bounds(&self) -> Bounds3f;

    /// Intersection test with world space `ray`.
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;

}
