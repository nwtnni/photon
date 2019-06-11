use crate::math;
use crate::scene;
use crate::geom;
use crate::geom::Surface;

mod normal;
mod point;
mod bsdf;

pub use normal::Normal;
pub use point::Point;
pub use bsdf::BSDF;

pub trait Integrator<'scene> {
    fn shade(scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Record<'scene>, depth: usize) -> math::Vec3;
}

pub fn shadowed<'scene>(scene: &scene::Scene<'scene>, point: &math::Vec3, light: &math::Vec3) -> bool {
    let wi = light - point;
    let mut shadow = math::Ray::new(*point, wi);
    shadow.set_max(wi.len());
    scene.hit_any(&shadow)
}
