use crate::math::{Ray, Vec3};
use crate::scene;
use crate::geom;

mod normal;
mod point;

pub use normal::Normal;
pub use point::Point;

pub trait Integrator<'scene> {
    fn shade(scene: &scene::Scene<'scene>, ray: &Ray, hit: &geom::Record<'scene>, depth: usize) -> Vec3;
}
