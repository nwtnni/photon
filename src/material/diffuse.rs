use crate::math::{Ray, Vec3, uniform_sphere};
use crate::surface;
use crate::material::Material;
use crate::texture::Texture;

#[derive(Copy, Clone, Debug)]
pub struct Diffuse<'scene> {
    albedo: &'scene dyn Texture,
}

impl<'scene> Diffuse<'scene> {
    pub fn new(albedo: &'scene Texture) -> Self {
        Diffuse { albedo }
    }
}

impl<'scene> Material<'scene> for Diffuse<'scene> {
    fn scatter(&self, _: &Ray, hit: &surface::Record<'scene>, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = hit.p + hit.n + uniform_sphere();
        *scattered = Ray::new(hit.p, target - hit.p);
        *attenuation = self.albedo.evaluate(hit.u, hit.v);
        true
    }
}
