use crate::prelude::*;
use crate::geom;
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

        for light in scene.lights() {

            let ls = light.sample(&p);

            if integrator::shadowed(scene, &p, &ls.d, ls.t) { continue }

            color += light.eval(&math::Ray::new(p, ls.d))
                * hit.bxdf.unwrap().eval(&ls.d, &wr, &n)
                * ls.a
                * n.dot(&ls.d)
                / if ls.p > 0.000_01 { ls.p } else { 1.0 };
        }

        let bs = hit.bxdf.unwrap().sample(&wr, &n);

        let mut hr = geom::Record::default();
        let mut recurse = math::Ray::new(p, bs.d);

        if bs.delta {
            if scene.hit(&mut recurse, &mut hr) {
                color += Self::shade(scene, &ray, &hr, depth + 1)
                    * bs.v
                    * n.dot(&bs.d)
                    / if bs.p > 0.000_01 { bs.p } else { 1.0 };
            }
        }

        color
    }
}
