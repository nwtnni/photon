use crate::geometry::{Ray, Vec3};
use crate::surface::{Surface, Hit};
use crate::material::Material;

/// Basic sphere.
#[derive(Copy, Clone, Debug)]
pub struct Sphere<'scene> {
    /// Center
    c: Vec3,

    /// Velocity
    v: Vec3,

    /// Radius
    r: f32,

    /// Material
    m: &'scene dyn Material,
}

impl<'scene> Sphere<'scene> {
    #[inline(always)]
    pub fn new(c: Vec3, r: f32, m: &'scene Material) -> Self {
        Sphere { c, r, m, v: Vec3::default() }
    }

    #[inline(always)]
    pub fn with_velocity(self, v: Vec3) -> Self {
        Sphere { v, .. self }
    }

    #[inline(always)]
    pub fn c(&self) -> Vec3 { self.c }

    #[inline(always)]
    pub fn r(&self) -> f32 { self.r }

    fn center(&self, t: f32) -> Vec3 {
        self.c + self.v * t
    }
}

impl<'scene> Surface<'scene> for Sphere<'scene> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit<'scene>) -> bool {
        let o = ray.o() - self.center(ray.t());
        let a = ray.d().len_sq() as f32;
        let b = o.dot(&ray.d());
        let c = o.len_sq() - self.r * self.r;
        let d = b * b - a * c;

        if d < 0.0 { return false }

        let (t_a, t_b) = ((-b - d.sqrt()) / a, (-b + d.sqrt()) / a);

        // Get first intersection within [t_min, t_max]
        let t = if t_a > t_min && t_a < t_max {
            t_a
        } else if t_b > t_min && t_a < t_max {
            t_b
        } else {
            return false;
        };

        hit.t = t;
        hit.p = ray.at(t);
        hit.n = (hit.p - self.c()) / self.r();
        hit.m = Some(self.m);
        true
    }
}
