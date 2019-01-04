use crate::math::{Point2f, Point3f, Normal3f, Vec3f};

#[derive(Clone)]
pub struct Intersection {
    /// Hit point
    pub p: Point3f,

    /// Surface normal
    pub n: Normal3f,

    /// Texture coordinates
    pub uv: Point2f,

    /// Derivative of point w.r.t. u texture coordinate
    pub dp_du: Vec3f,

    /// Derivative of point w.r.t. v texture coordinate
    pub dp_dv: Vec3f,
}
