use crate::integrator;
use crate::geom;
use crate::math::{Ray, Vec3};
use crate::scene;

pub struct Point {

}

impl<'scene> integrator::Integrator<'scene> for Point {
    fn shade(scene: &scene::Scene<'scene>, ray: &Ray, hit: &geom::Record<'scene>, depth: usize) -> Vec3 {

        let mut color = Vec3::default();

        for light in scene.lights() {
            // Filter point lights
        }

        color
    }
}
