use crate::prelude::*;
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

        let bxdf = hit.bxdf.unwrap().sample(&wr, &n);

        let mut hit_record = geom::Record::default();
        let mut ray = math::Ray::new(p, bxdf.d);

        if bxdf.delta {
            if scene.hit(&mut ray, &mut hit_record) {
                color += Self::shade(scene, &ray, &hit_record, depth + 1)
                    * bxdf.v
                    * n.dot(&bxdf.d)
                    / if bxdf.p > 0.000_01 { bxdf.p } else { 1.0 };
            }
        }

        color
    }
}
