use crate::bxdf;
use crate::math;
use crate::math::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    color: Vec3,
}

impl bxdf::BXDF for Lambertian {
    fn eval(&self, wi: &Vec3, wo: &Vec3, n: &Vec3, out: &mut Vec3) {
        *out = if wi.dot(n) >= 0.0 && wo.dot(n) >= 0.0 {
            self.color / math::PI
        } else {
            Vec3::default()
        };
    }

    fn sample(&self, sample: &mut bxdf::Record, out: &mut Vec3) -> f32 {
        let local = math::cosine_sphere();  
        let (u, v) = math::basis(&sample.n);
        sample.wo = sample.n * local.z();
        sample.wo += u * local.x();
        sample.wo += v * local.y();
        *out = self.color / math::PI;
        sample.wo.dot(&sample.n) / math::PI
    }

    fn pdf(&self, _: &Vec3, wo: &Vec3, n: &Vec3) -> f32 {
        if wo.dot(n) >= 0.0 { wo.dot(n) / math::PI } else { 0.0 }
    }
}
