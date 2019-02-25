use crate::geometry::{Ray, Vec3};
use crate::material::{Material, reflect, refract};
use crate::surface::Hit;

#[derive(Copy, Clone, Debug, Default)]
pub struct Dielectric {
    index: f32,
}

impl Dielectric {
    pub fn new(index: f32) -> Self {
        Dielectric { index }
    }
}

impl Material for Dielectric {
    fn scatter<'scene>(&self, ray: &Ray, hit: &'scene Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(ray.d(), hit.n);
        *attenuation = Vec3::new(1.0, 1.0, 1.0);

        let (outward, ni_over_nt) = if ray.d().dot(&hit.n) > 0.0 {
            (-hit.n, self.index)
        } else {
            (hit.n, 1.0 / self.index)
        };

        if let Some(refracted) = refract(ray.d(), outward, ni_over_nt) {
            *scattered = Ray::new(hit.p, refracted);
        } else {
            *scattered = Ray::new(hit.p, reflected);
        }

        true
    }
}
