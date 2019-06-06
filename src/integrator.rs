use crate::geom::{Ray, Vec3};
use crate::surface::Hit;
use crate::scene;

mod normal;

pub use normal::Normal;

pub trait Integrator<'scene> {
    fn shade(scene: &scene::Scene<'scene>, ray: &Ray, hit: &Hit<'scene>, depth: usize) -> Vec3;
}
