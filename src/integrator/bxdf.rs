use std::ops::Sub;

use crate::bxdf::BxDF as _;
use crate::prelude::*;
use crate::geom;
use crate::math;
use crate::scene;
use crate::integrator;

#[derive(Copy, Clone, Debug)]
pub struct BxDF;

impl<'scene> integrator::Integrator<'scene> for BxDF {
    fn shade(&self, scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Hit<'scene>, depth: usize) -> math::Vec3 {
        let p = hit.p;
        let n = hit.n;  
        let wr = (ray.p - hit.p).normalize();

        let mut color = hit.emit.unwrap_or_default();

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

        let bs = hit.bxdf.unwrap().sample(&wr, &n);

        let mut hit_record = geom::Hit::default();
        let mut ray = math::Ray::new(p, bs.d);

        if scene.hit(&mut ray, &mut hit_record) {
            if bs.delta {
                color += self.shade(scene, &ray, &hit_record, depth + 1);
            } else if let Some(light) = hit_record.emit {
                color += light; 
            }
        }

        color * bs.v
            * bs.d.dot(&n).abs()
            / if bs.p > 0.000_01 { bs.p } else { 1.0 }
    }
}
