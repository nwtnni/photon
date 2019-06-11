use std::ops::Sub;

use crate::prelude::*;
use crate::bxdf;
use crate::geom;
use crate::light;
use crate::math;
use crate::scene;
use crate::integrator;

pub struct Light;

impl<'scene> integrator::Integrator<'scene> for Light {
    fn shade(scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Record<'scene>, depth: usize) -> math::Vec3 {

        let p = hit.p;
        let n = hit.n;  
        let wr = (ray.p - hit.p).normalize();

        let mut color = hit.emit.unwrap_or_default() + scene.background();
        let mut light_record = light::Record::default();

        for light in scene.lights() {

            light.sample(&p, &mut light_record);

            let l = light_record.l;

            if integrator::shadowed(scene, &p, &l) {
                continue
            }

            let wi = (l - p).normalize();
            let ray = math::Ray::new(p, wi);


            color += light.eval(&ray)
                * hit.bxdf.unwrap().eval(&wi, &wr, &n)
                * light_record.a
                * n.dot(&wi)
                / if light_record.p > 0.000_01 { light_record.p } else { 1.0 };
        }

        let mut bxdf_record = bxdf::Record::default();
        hit.bxdf.unwrap().sample(&wr, &n, &mut bxdf_record);

        let mut hit_record = geom::Record::default();
        let mut ray = math::Ray::new(p, bxdf_record.w);

        if bxdf_record.discrete {
            if scene.hit(&mut ray, &mut hit_record) {
                color += Self::shade(scene, &ray, &hit_record, depth + 1)
                    * bxdf_record.bxdf
                    * n.dot(&bxdf_record.w)
                    / if bxdf_record.probability > 0.000_01 { bxdf_record.probability } else { 1.0 };
            }
        }

        color
    }
}
