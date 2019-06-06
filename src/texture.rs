mod checker;
mod constant;

pub use constant::Constant;
pub use checker::Checker;

use crate::geom::Vec3;

pub trait Texture: std::fmt::Debug + Send + Sync {
    fn evaluate(&self, u: f32, v: f32) -> Vec3;
}
