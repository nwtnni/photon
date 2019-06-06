use crate::math::{Ray, Vec3};
use crate::material::{Material, reflect, refract};
use crate::surface;

#[derive(Copy, Clone, Debug, Default)]
pub struct Dielectric {
    index: f32,
}

impl Dielectric {
    pub fn new(index: f32) -> Self {
        Dielectric { index }
    }

    fn shlick(&self, cosine: f32) -> f32 {
        let r = (1.0 - self.index) / (1.0 + self.index);
        let r = r * r;
        r + (1.0 - r) * (1.0 - cosine).powf(5.0)
    }
}

impl<'scene> Material<'scene> for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &surface::Record<'scene>, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let reflected = reflect(ray.d, hit.n);
        let dot = ray.d.dot(&hit.n);
        let cosine;
        let outward;
        let ni_over_nt;

        if dot > 0.0 {
            cosine = self.index * dot / ray.d.len();
            outward = -hit.n;
            ni_over_nt = self.index;
        } else {
            cosine = -dot / ray.d.len();
            outward = hit.n;
            ni_over_nt = 1.0 / self.index;
        };

        if let Some(refracted) = refract(ray.d, outward, ni_over_nt) {
            if rand::random::<f32>() >= self.shlick(cosine) {
                *scattered = Ray::new(hit.p, refracted);
                return true
            }
        }

        *scattered = Ray::new(hit.p, reflected);
        true
    }
}
