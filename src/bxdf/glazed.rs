use crate::bxdf;
use crate::math;

#[derive(Copy, Clone, Debug)]
pub struct Glazed<'scene> {
    bxdf: &'scene bxdf::Any<'scene>, 
    eta: f32,
}

impl<'scene> Glazed<'scene> {
    pub fn new(bxdf: &'scene bxdf::Any<'scene>, eta: f32) -> Self {
        Glazed { bxdf, eta }
    }
}

impl<'scene> bxdf::BxDF for Glazed<'scene> {
    fn eval(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> math::Vec3 {
        self.bxdf.eval(wi, wr, n)
    }

    fn sample(&self, wi: &math::Vec3, n: &math::Vec3) -> bxdf::Sample {
        let fresnel = bxdf::Fresnel::dieletric(wi, n, self.eta);
        if rand::random::<f32>() <= fresnel.reflect {
            fresnel.reflect()
        } else {
            let d = math::cosine_sphere();
            let (u, v) = math::basis(n);
            let wr = n * d.z() + u * d.x() + v * d.y();
            let v = self.bxdf.eval(wi, &wr, n) * (1.0 - fresnel.reflect);
            let p = (1.0 - fresnel.reflect) * (fresnel.cos_i / math::PI);
            bxdf::Sample { d: wr, v, p, delta: false }
        }
    }

    fn pdf(&self, _: &math::Vec3, _: &math::Vec3, _: &math::Vec3) -> f32 {
        0.0
    }
}
