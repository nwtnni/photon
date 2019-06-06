use crate::material::Material;
use crate::math::{Vec3, Ray, uniform_sphere};
use crate::geom;

#[derive(Copy, Clone, Debug)]
pub struct Normal;

impl<'scene> Material<'scene> for Normal {
    fn scatter(&self, _: &Ray, hit: &geom::Record<'scene>, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = hit.p + hit.n + uniform_sphere();
        *scattered = Ray::new(hit.p, target - hit.p);
        *attenuation = hit.n.normalize();
        true
    }
}
