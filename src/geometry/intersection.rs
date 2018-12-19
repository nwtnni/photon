use std::sync::Arc;

use noisy_float::prelude::*;

use crate::math::{Point2f, Point3f, Normal3f, Vec3f};
use crate::geometry::Shape;

#[derive(Clone)]
pub struct Intersection {
    /// Hit point
    p: Point3f,

    /// Surface normal at hit point
    n: Normal3f,

    /// Texture coordinates of hit point
    uv: Point2f,
}

impl Intersection {
    pub fn new(p: Point3f, n: Normal3f, uv: Point2f) -> Self {
        Intersection { p, n, uv }
    }
}
