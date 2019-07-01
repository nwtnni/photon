use crate::prelude::*;
use crate::geom;
use crate::math;
use crate::scene;
use crate::integrator;

#[derive(Copy, Clone, Debug)]
pub struct Light;

impl<'scene> integrator::Integrator<'scene> for Light {
    fn shade(&self, scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Record<'scene>, depth: usize) -> math::Vec3 {

        if depth > 5 { return math::Vec3::default() }

        let p = hit.p;
        let n = hit.n;  
        let wr = (ray.p - hit.p).normalize();

        let mut color = hit.emit.unwrap_or_default();

        for light in scene.lights() {

            let ls = light.sample(&p);

            if integrator::shadowed(scene, &p, &ls.d, ls.t) { continue }

            color += light.eval(&math::Ray::new(p, ls.d))
                * hit.bxdf.unwrap().eval(&ls.d, &wr, &n)
                * ls.a
                * n.dot(&ls.d)
                / ls.p;
        }

        let bs = hit.bxdf.unwrap().sample(&wr, &n);

        if bs.delta && bs.p > 0.001 {
            let mut hr = geom::Record::default();
            let mut recurse = math::Ray::new(p, bs.d);

            if !scene.hit(&mut recurse, &mut hr) { return color }

            color += self.shade(scene, &recurse, &hr, depth + 1)
                * bs.v
                * n.dot(&bs.d).abs()
                / bs.p
        }

        color
    }
}
