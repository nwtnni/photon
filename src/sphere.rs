use crate::{Ray, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    c: Vec3,
    r: f32,
}

impl Sphere {
    #[inline(always)]
    pub fn new(c: Vec3, r: f32) -> Self {
        Sphere { c, r }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let oc = ray.o() - self.c;
        let a = ray.d().len_sq() as f32;
        let b = 2.0 * oc.dot(&ray.d());
        let c = oc.len_sq() - self.r * self.r;
        b * b - 4.0 * a * c > 0.0
    }
}
