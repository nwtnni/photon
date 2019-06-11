use crate::math;
use crate::scene;
use crate::geom;
use crate::geom::Surface;

mod normal;
mod point;
mod bsdf;
mod light;

pub use normal::Normal;
pub use point::Point;
pub use bsdf::BSDF;
pub use light::Light;

pub trait Integrator<'scene> {
    fn shade(scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Record<'scene>, depth: usize) -> math::Vec3;
}

pub fn shadowed<'scene>(scene: &scene::Scene<'scene>, p: &math::Vec3, d: &math::Vec3, t: f32) -> bool {
    let mut shadow = math::Ray::new(*p, *d);
    shadow.set_max(t);
    scene.hit_any(&shadow)
}
