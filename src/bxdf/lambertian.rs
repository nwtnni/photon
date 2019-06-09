use crate::bxdf;
use crate::math;
use crate::math::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    color: Vec3,
}

impl Lambertian {
    pub fn new(color: Vec3) -> Self {
        Lambertian { color }
    }
}

impl bxdf::BxDF for Lambertian {
    fn eval(&self, wi: &Vec3, wr: &Vec3, n: &Vec3) -> Vec3 {
        if wi.dot(n) >= 0.0 && wr.dot(n) >= 0.0 {
            self.color / math::PI
        } else {
            Vec3::default()
        }
    }

    fn sample(&self, _: &Vec3, n: &Vec3, sample: &mut bxdf::Record) {
        let local = math::cosine_sphere();  
        let (u, v) = math::basis(n);
        sample.w = n * local.z()
            + u * local.x()
            + v * local.y();
        sample.bxdf = self.color / math::PI;
        sample.discrete = false;
        sample.probability = sample.w.dot(n) / math::PI;
    }

    fn pdf(&self, _: &Vec3, wr: &Vec3, n: &Vec3) -> f32 {
        if wr.dot(n) >= 0.0 { wr.dot(n) / math::PI } else { 0.0 }
    }
}
