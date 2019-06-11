use std::ops::Sub;

use crate::prelude::*;
use crate::bxdf;
use crate::geom;
use crate::math;
use crate::scene;
use crate::integrator;

pub struct BSDF;

impl<'scene> integrator::Integrator<'scene> for BSDF {
    fn shade(scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Record<'scene>, depth: usize) -> math::Vec3 {

        let p = hit.p;
        let n = hit.n;  
        let wr = (ray.p - hit.p).normalize();

        let mut color = hit.emit.unwrap_or_default();

        for light in scene.lights().iter().filter_map(|light| light.downcast_point()) {

            let l = light.p;
            let wi = (l - hit.p).normalize();

            if integrator::shadowed(scene, &p, &l) || n.dot(&wi) < 0.0 { continue }

            color += hit.bxdf.unwrap().eval(&wi, &wr, &n)
                / l.sub(&p).len_sq()
                * n.dot(&wi)
                * light.i;
        }

        let bxdf = hit.bxdf.unwrap().sample(&wr, &n);

        let mut hit_record = geom::Record::default();
        let mut ray = math::Ray::new(p, bxdf.d);

        if scene.hit(&mut ray, &mut hit_record) {
            if bxdf.delta {
                color += Self::shade(scene, &ray, &hit_record, depth + 1);
            } else if let Some(light) = hit_record.emit {
                color += light; 
            }
        } else {
            color += scene.background();
        }

        color *= bxdf.v * bxdf.d.dot(&n).abs();
        if bxdf.p > 0.000_01 { color /= bxdf.p; }
        color
    }
}
