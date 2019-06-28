use crate::bxdf;
use crate::math;

#[derive(Copy, Clone, Debug)]
pub struct Mirror;

impl bxdf::BxDF for Mirror {
    fn eval(&self, _: &math::Vec3, _: &math::Vec3, _: &math::Vec3) -> math::Vec3 {
        math::Vec3::default()
    }

    fn sample(&self, d: &math::Vec3, n: &math::Vec3) -> bxdf::Sample {
        let fresnel = bxdf::Fresnel::mirror(d, n);
        fresnel.reflect()
    }

    fn pdf(&self, _: &math::Vec3, _: &math::Vec3, _: &math::Vec3) -> f32 {
        0.0
    }
}
