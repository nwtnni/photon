use crate::geometry::{Ray, Vec3, uniform_sphere};
use crate::material::{Material, reflect};
use crate::surface::Hit;

#[derive(Copy, Clone, Debug, Default)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let fuzz = if fuzz > 1.0 { 1.0 } else { fuzz };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let d = ray.d().normalize();
        *attenuation = self.albedo;
        *scattered = Ray::new(
            hit.p,
            reflect(d, hit.n) + uniform_sphere() * self.fuzz,
            ray.t(),
        );
        scattered.d().dot(&hit.n) > 0.0
    }
}
