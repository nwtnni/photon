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

impl<'a, 'scene, I> Integrator<'scene> for &'a I where I: Integrator<'scene> + ?Sized {
    fn shade(&self, scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Hit<'scene>, depth: usize) -> math::Vec3 {
        (*self).shade(scene, ray, hit, depth)
    }
}

pub fn shadowed<'scene>(scene: &scene::Scene<'scene>, p: &math::Vec3, d: &math::Vec3, t: f32) -> bool {
    let mut shadow = math::Ray::new(*p, *d);
    shadow.set_max(t);
    scene.hit_any(&shadow)
}

#[derive(Copy, Clone, Debug)]
pub enum Any {
    Normal(Normal),
    Point(Point),
    Path(Path),
    BxDF(BxDF),
    Light(Light),
}

impl<'scene> Integrator<'scene> for Any {
    fn shade(&self, scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Hit<'scene>, depth: usize) -> math::Vec3 {
        match self {
            Any::Normal(integrator) => integrator.shade(scene, ray, hit, depth),
            Any::Point(integrator) => integrator.shade(scene, ray, hit, depth),
            Any::Path(integrator) => integrator.shade(scene, ray, hit, depth),
            Any::BxDF(integrator) => integrator.shade(scene, ray, hit, depth),
            Any::Light(integrator) => integrator.shade(scene, ray, hit, depth),
        }
    }
}
