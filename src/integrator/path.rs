use crate::bxdf::BxDF;
use crate::geom;
use crate::math;
use crate::scene;
use crate::integrator;

use crate::geom::Surface;

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
    fn shade(&self, scene: &scene::Scene<'scene>, ray: &math::Ray, hit: &geom::Hit<'scene>, _: usize) -> math::Vec3 {

        let mut beta = math::Vec3::broadcast(1.0);
        let mut color = math::Vec3::default();

        let mut ray = ray.clone();
        let mut hit = hit.clone();
        let mut specular_bounce = false;

        for bounces in 0.. {

            if bounces == 0 || specular_bounce {
                if let Some(emit) = hit.emit {
                    color += emit * beta;
                }
            }

            let d = (ray.p - hit.p).normalize();

            // Sample lights

            let mut l = math::Vec3::default();

            for light in scene.lights() {

                let ls = light.sample(&hit.p);

                if integrator::shadowed(scene, &hit.p, &ls.d, ls.t) { continue }

                l += light.eval(&math::Ray::new(hit.p, ls.d))
                    * hit.bxdf.unwrap().eval(&d, &ls.d, &hit.n)
                    * ls.a
                    * hit.n.dot(&ls.d)
                    / ls.p;
            }

            color += l * beta;

            // Sample BSDF

            let bs = hit.bxdf.unwrap().sample(&d, &hit.n);

            if bs.p < math::EPSILON || bs.v.is_zero() {
                break
            }

            beta *= bs.v * d.dot(&hit.n).abs() / bs.p;

            specular_bounce = bs.delta;

            // Russian roulette termination

            if bounces > 3 {
                let q = math::max(0.05, 1.0 - beta.y());
                if rand::random::<f32>() < q { break }
                beta /= 1.0 - q;
            }

            ray = math::Ray::new(hit.p, bs.d);

            if bounces >= self.depth || beta.y() > self.threshold || !scene.hit(&mut ray, &mut hit) {
                break
            }
        }

        color
    }
}
