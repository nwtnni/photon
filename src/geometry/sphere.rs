use noisy_float::prelude::*;

use crate::math::{Normal3f, Point2f, Point3f, Ray, Vec3, clamp, solve_quadratic};
use crate::geometry::*;

pub struct Sphere {
    radius: N32,
}

impl Shape for Sphere {
    fn bounds(&self) -> Bounds3f {
        Bounds3f::new(
            Point3f::new(-self.radius, -self.radius, -self.radius),
            Point3f::new(self.radius, self.radius, self.radius),
        )
    }

    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let o = Vec3::from(ray.o());
        let a = ray.d().len_sq();
        let b = ray.d().dot_v(&o) * n32(2.0);
        let c = o.len_sq() - self.radius;
        let t = solve_quadratic(a, b, c)?;
        if t.0 > ray.max() || t.1 < ray.min() {
            return None
        }

        let mut t_hit = t.0;
        if t_hit < n32(0.0) {
            t_hit = t.1;
            if t_hit > ray.max() {
                return None
            }
        }

        let p = ray.get(t_hit);
        let n = Normal3f::new(p.x(), p.y(), p.z());
        let v = clamp(p.z() / self.radius, n32(-1.0), n32(1.0));
        let u = match N32::atan2(p.y(), p.x()) / n32(2.0 * std::f32::consts::PI) {
        | u if u < 0.0 => u + 1.0,
        | u => u,
        };

        Some(Intersection::new(p, n, Point2f::new(u, v)))
    }
}
