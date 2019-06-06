use crate::geom::{Ray, Vec3};
use crate::scene;
use crate::surface;

mod normal;

pub use normal::Normal;

pub trait Integrator<'scene> {
    fn shade(scene: &scene::Scene<'scene>, ray: &Ray, hit: &surface::Record<'scene>, depth: usize) -> Vec3;
}
