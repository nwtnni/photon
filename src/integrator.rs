use std::fmt;

use crate::math;
use crate::scene;
use crate::geom;
use crate::geom::Surface;

mod normal;
mod point;
mod path;
mod bxdf;
mod light;

pub use normal::Normal;
pub use point::Point;
pub use path::Path;
pub use bxdf::BxDF;
pub use light::Light;

pub trait Integrator<'scene>: Send + Sync + fmt::Debug {
    fn shade(&self, scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Hit<'scene>, depth: usize) -> math::Vec3;
}

pub fn shadowed<'scene>(scene: &scene::Scene<'scene>, p: &math::Vec3, d: &math::Vec3, t: f32) -> bool {
    let mut shadow = math::Ray::new(*p, *d);
    shadow.set_max(t);
    scene.hit_any(&shadow)
}
