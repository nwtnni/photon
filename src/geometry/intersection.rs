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

    /// Time of hit
    t: N32,

    /// Negative ray direction
    wo: Vec3f,

    /// Texture coordinates of hit point
    uv: Point2f,

    /// Partial derivative of hit point `p` w.r.t. `u`
    dp_du: Vec3f,

    /// Partial derivative of hit point `p` w.r.t `v`
    dp_dv: Vec3f,

    /// Partial derivative of normal `n` w.r.t texture coordinate `u`
    dn_du: Normal3f,

    /// Partial derivative of normal `n` w.r.t texture coordinate `v`
    dn_dv: Normal3f,

    /// Pointer to hit Shape
    shape: Arc<dyn Shape>,
}
