use std::ops::Sub;

use crate::geom;
use crate::integrator;
use crate::math::{Ray, Vec3};
use crate::scene;

#[derive(Copy, Clone, Debug)]
pub struct Point;

impl<'scene> integrator::Integrator<'scene> for Point {
    fn shade(&self, scene: &scene::Scene<'scene>, ray: &Ray, hit: &geom::Hit<'scene>, _: usize) -> Vec3 {

        let p = hit.p;
        let n = hit.n;
        let wr = (ray.p - p).normalize();

        let mut color = Vec3::default();

        for light in scene.lights().iter().filter_map(|light| light.downcast_point()) {

            let l = light.p;
            let wi = (l - p).normalize();
            let t = (l - p).len();

            if integrator::shadowed(scene, &p, &wi, t) || n.dot(&wi) < 0.0 { continue }

            color += hit.bxdf.unwrap().eval(&wi, &wr, &n)
                / l.sub(&p).len_sq()
                * n.dot(&wi)
                * light.i;
        }

        color
    }
}
