use crate::geom;
use crate::math;
use crate::scene;
use crate::integrator;

#[derive(Copy, Clone, Debug)]
pub struct Path {
    /// Maximum recursion depth
    depth: usize, 

    /// Russian roulette termination threshold
    threshold: f32,
}

impl Path {
    pub fn new(depth: usize, threshold: f32) -> Self {
        Path {
            depth,
            threshold,
        }
    }
}

impl<'scene> integrator::Integrator<'scene> for Path {
    fn shade(&self, scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Record<'scene>, depth: usize) -> math::Vec3 {

        let mut beta = 1.0;
        let mut color = math::Vec3::default();

        for bounces in 0.. {
        
            // Sample light

            // Sample BSDF

            // Russian roulette termination

        }

        unimplemented!()

    }
}
