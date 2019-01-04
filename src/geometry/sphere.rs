use noisy_float::prelude::*;

use crate::math::{Normal3f, Point2f, Point3f, Ray, Vec3f, clamp, solve_quadratic};
use crate::geometry::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Sphere {
    radius: N32,
}

impl Sphere {
    #[inline]
    fn new(radius: N32) -> Self {
        Sphere { radius }
    }
}

impl Shape for Sphere {
    fn bounds(&self) -> Bounds3f {
        Bounds3f::new(
            Point3f::new(-self.radius, -self.radius, -self.radius),
            Point3f::new(self.radius, self.radius, self.radius),
        )
    }

    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        let o = Vec3f::from(ray.o());
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

        let phi = p.y().atan2(p.x());
        let phi = if phi < n32(0.0) { phi + n32(2.0 * std::f32::consts::PI) } else { phi };
        let theta = clamp(p.z() / self.radius, n32(-1.0), n32(1.0)).acos();

        let uv = Point2f::new(
            phi / n32(2.0 * std::f32::consts::PI),
            theta / n32(std::f32::consts::PI),
        );

        let inv_z = n32(1.0) / (p.x() * p.x() + p.y() * p.y());
        let cos_phi = p.x() * inv_z;
        let sin_phi = p.y() * inv_z;
        let dp_du = Vec3f::new(
            n32(-2.0 * std::f32::consts::PI) * p.y(),
            n32(2.0 * std::f32::consts::PI) * p.x(),
            n32(0.0),
        );
        let dp_dv = Vec3f::new(
            p.z() * cos_phi,
            p.z() * sin_phi,
            -self.radius * theta.sin(),
        ) * n32(std::f32::consts::PI);

        Some(Intersection {
            p,
            n,
            uv,
            dp_du,
            dp_dv,
        })
    }
}
