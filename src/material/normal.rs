use crate::material::Material;
use crate::geometry::{Vec3, Ray, uniform_sphere};
use crate::surface::Hit;

#[derive(Copy, Clone, Debug)]
pub struct Normal;

impl<'scene> Material<'scene> for Normal {
    fn scatter(&self, ray: &Ray, hit: &Hit<'scene>, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = hit.p + hit.n + uniform_sphere();
        *scattered = Ray::new(hit.p, target - hit.p, ray.t);
        *attenuation = hit.n.normalize();
        true
    }
}
