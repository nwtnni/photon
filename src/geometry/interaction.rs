use std::sync::Arc;

use crate::math::{Point2f, Normal3f, Vec3f};
use crate::geometry::Shape;

#[derive(Clone, Debug)]
pub struct Interaction {
    uv: Point2f,
    dp_du: Vec3f,
    dp_dv: Vec3f,
    dn_du: Normal3f,
    dn_dv: Normal3f,
    shape: Arc<dyn Shape>,
}
