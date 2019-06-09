use std::ops::Sub;

use crate::prelude::*;
use crate::geom;
use crate::integrator;
use crate::math::{Ray, Vec3};
use crate::scene;

#[derive(Copy, Clone, Debug)]
pub struct Point;

impl<'scene> integrator::Integrator<'scene> for Point {
    fn shade(scene: &scene::Scene<'scene>, ray: &Ray, hit: &geom::Record<'scene>, _: usize) -> Vec3 {

        let mut color = Vec3::default();

        for light in scene.lights().iter().filter_map(|light| light.downcast_point()) {
            let p = hit.p;
            let n = hit.n;
            let l = light.p();
            let wi = (l - p).normalize();
            let wr = (ray.o - p).normalize();

            if scene.hit_any(&Ray::new(p, wi)) || n.dot(&wi) < 0.0 { continue }

            color += hit.bxdf.unwrap().eval(&wi, &wr, &n)
                / l.sub(&p).len_sq()
                * n.dot(&wi)
                * light.i();
        }

        color
    }
}
