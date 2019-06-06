use crate::bxdf;
use crate::geom;
use crate::geom::{basis, cosine_sphere, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    color: Vec3,
}

impl bxdf::BXDF for Lambertian {
    fn eval(&self, wi: &Vec3, wo: &Vec3, n: &Vec3, out: &mut Vec3) {
        *out = if wi.dot(n) >= 0.0 && wo.dot(n) >= 0.0 {
            self.color / geom::PI
        } else {
            Vec3::default()
        };
    }

    fn sample(&self, wi: &Vec3, sample: &mut bxdf::Sample, out: &mut Vec3) -> f32 {
        let local = cosine_sphere();  
        let (u, v) = basis(&sample.n);
        sample.wo = sample.n * local.z();
        sample.wo += u * local.x();
        sample.wo += v * local.y();
        *out = self.color / geom::PI;
        sample.wo.dot(&sample.n) / geom::PI
    }

    fn pdf(&self, wi: &Vec3, wo: &Vec3, n: &Vec3) -> f32 {
        if wo.dot(n) >= 0.0 { wo.dot(n) / geom::PI } else { 0.0 }
    }
}
