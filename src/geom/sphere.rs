use crate::bxdf;
use crate::math::{Ray, Vec3};
use crate::geom;

/// Basic sphere.
#[readonly::make]
#[derive(Copy, Clone, Debug)]
pub struct Sphere<'scene> {
    /// Center
    pub center: Vec3,

    /// Radius
    pub radius: f32,

    /// BXDF
    pub bxdf: &'scene dyn bxdf::BXDF,
}

impl<'scene> Sphere<'scene> {
    pub fn new(center: Vec3, radius: f32, bxdf: &'scene dyn bxdf::BXDF) -> Self {
        Sphere { center, radius, bxdf }
    }
}

impl<'scene> geom::Surface<'scene> for Sphere<'scene> {
    fn bound(&self) -> geom::Box3 {
        let r = Vec3::broadcast(self.radius);
        geom::Box3::new(self.center - r, self.center + r)
    }

    fn hit(&self, ray: &mut Ray, hit: &mut geom::Record<'scene>) -> bool {

        if cfg!(feature = "stats") {
            crate::stats::INTERSECTION_TESTS.inc();
            crate::stats::SPHERE_INTERSECTION_TESTS.inc();
        }

        let o = ray.origin - self.center;
        let a = ray.dir.len_sq() as f32;
        let b = o.dot(&ray.dir);
        let c = o.len_sq() - self.radius * self.radius;
        let d = b * b - a * c;

        if d < 0.0 { return false }

        let (t_a, t_b) = ((-b - d.sqrt()) / a, (-b + d.sqrt()) / a);

        // Get first intersection within [t_min, t_max]
        let t = if t_a > ray.min && t_a < ray.max {
            t_a
        } else if t_b > ray.min && t_b < ray.max {
            t_b
        } else {
            return false;
        };

        ray.set_max(t);
        hit.t = t;
        hit.p = ray.at(t);
        hit.n = (hit.p - self.center) / self.radius;
        hit.bxdf = Some(self.bxdf);
        let phi = hit.p.z().atan2(hit.p.x());
        let theta = hit.p.y().asin();
        hit.u = 1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
        hit.v = (theta + std::f32::consts::FRAC_PI_2) / std::f32::consts::PI;
        true
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        if cfg!(feature = "stats") {
            crate::stats::INTERSECTION_TESTS.inc();
            crate::stats::SPHERE_INTERSECTION_TESTS.inc();
        }
        let o = ray.origin - self.center;
        let a = ray.dir.len_sq() as f32;
        let b = o.dot(&ray.dir);
        let c = o.len_sq() - self.radius * self.radius;
        let d = b * b - a * c;
        if d < 0.0 { return false }
        let (t_a, t_b) = ((-b - d.sqrt()) / a, (-b + d.sqrt()) / a);
        t_a > ray.min && t_a < ray.max || t_b > ray.min && t_a < ray.max
    }
}
