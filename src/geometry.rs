mod bounds;
mod intersection;
mod sphere;

pub use self::bounds::*;
pub use self::intersection::*;
pub use self::sphere::*;

use noisy_float::prelude::*;

use crate::math::{Ray, Transform};

/// Represents a geometric shape that can be intersected with a ray.
pub trait Shape {

    /// Bounding box in object space.
    fn bounds(&self) -> Bounds3f;

    /// Intersection test with object space `ray`.
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;

}
