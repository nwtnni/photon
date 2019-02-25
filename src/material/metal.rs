use crate::{Vec3, Hit, Ray, Material, uniform_sphere};

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
        let reflected = d - hit.n * 2.0 * d.dot(&hit.n);
        *scattered = Ray::new(hit.p, reflected + uniform_sphere() * self.fuzz);
        *attenuation = self.albedo;
        scattered.d().dot(&hit.n) > 0.0
    }
}
