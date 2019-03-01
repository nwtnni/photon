use crate::geometry::{Ray, Vec3, uniform_sphere};
use crate::surface::Hit;
use crate::material::Material;

#[derive(Copy, Clone, Debug, Default)]
pub struct Diffuse {
    albedo: Vec3,
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Self {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter<'scene>(&self, ray: &Ray, hit: &'scene Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = hit.p + hit.n + uniform_sphere();
        *scattered = Ray::new(hit.p, target - hit.p, ray.t());
        *attenuation = self.albedo;
        true
    }
}
