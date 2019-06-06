use crate::math::{Ray, Vec3};
use crate::scene;
use crate::geom;

mod normal;

pub use normal::Normal;

pub trait Integrator<'scene> {
    fn shade(scene: &scene::Scene<'scene>, ray: &Ray, hit: &geom::Record<'scene>, depth: usize) -> Vec3;
}
