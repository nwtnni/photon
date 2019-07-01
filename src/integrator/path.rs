use crate::geom;
use crate::math;
use crate::scene;
use crate::integrator;

#[derive(Copy, Clone, Debug)]
pub struct Path;

impl<'scene> integrator::Integrator<'scene> for Path {
    fn shade(&self, scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Record<'scene>, depth: usize) -> math::Vec3 {
        unimplemented!()
    }
}
