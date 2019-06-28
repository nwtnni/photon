use crate::bxdf;
use crate::math;

#[derive(Copy, Clone, Debug)]
pub struct Specular {
    color: math::Vec3,
    eta: f32,
}

impl Specular {
    pub fn new(color: math::Vec3, eta: f32) -> Self {
        Specular { color, eta }
    }
}

impl bxdf::BxDF for Specular {
    fn eval(&self, _: &math::Vec3, _: &math::Vec3, _: &math::Vec3) -> math::Vec3 {
        math::Vec3::default()
    }

    fn sample(&self, d: &math::Vec3, n: &math::Vec3) -> bxdf::Sample {
        let fresnel = bxdf::Fresnel::dieletric(d, n, self.eta);
        if rand::random::<f32>() <= fresnel.reflect {
            fresnel.reflect()
        } else {
            fresnel.refract()
        }
    }

    fn pdf(&self, _: &math::Vec3, _: &math::Vec3, _: &math::Vec3) -> f32 {
        0.0
    }
}
