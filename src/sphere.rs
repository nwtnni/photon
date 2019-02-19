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

    #[inline(always)]
    pub fn c(&self) -> Vec3 { self.c }

    #[inline(always)]
    pub fn r(&self) -> f32 { self.r }

    pub fn intersects(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.o() - self.c;
        let a = ray.d().len_sq() as f32;
        let b = 2.0 * oc.dot(&ray.d());
        let c = oc.len_sq() - self.r * self.r;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}
