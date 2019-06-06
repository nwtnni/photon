use crate::geom::{Ray, Vec3, uniform_sphere};
use crate::material::{Material, reflect};
use crate::surface;

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

impl<'scene> Material<'scene> for Metal {
    fn scatter(&self, ray: &Ray, hit: &surface::Record<'scene>, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let d = ray.d.normalize();
        *attenuation = self.albedo;
        *scattered = Ray::new(
            hit.p,
            reflect(d, hit.n) + uniform_sphere() * self.fuzz,
        );
        scattered.d.dot(&hit.n) > 0.0
    }
}
