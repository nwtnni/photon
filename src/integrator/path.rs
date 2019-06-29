use crate::geom;
use crate::math;
use crate::scene;
use crate::integrator;

pub struct Path;

impl<'scene> integrator::Integrator<'scene> for Path {
    fn shade(scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Record<'scene>, depth: usize) -> math::Vec3 {
        unimplemented!()
    }
}
