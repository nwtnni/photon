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

    fn sample(&self, w: &math::Vec3, n: &math::Vec3, sample: &mut bxdf::Record) {
        let cos_i = w.dot(n);

        let (cos_i, eta, n) = if cos_i <= 0.0 {
            (-cos_i, 1.0 / self.eta, -n)
        } else {
            (cos_i, self.eta, *n)
        };

        let (reflect, cos_t) = bxdf::dieletric(cos_i, eta);

        if rand::random::<f32>() < reflect {
            sample.w = math::reflect(*w, n).normalize();
            sample.bxdf = math::Vec3::broadcast(reflect / cos_i);
            sample.probability = reflect;
            sample.discrete = true;
        } else {
            sample.w = ((n * cos_i - w) * (1.0 / eta) + (n * -cos_t)).normalize();
            sample.bxdf = math::Vec3::broadcast((1.0 - reflect) / cos_t);
            sample.probability = 1.0 - reflect;
            sample.discrete = true;
        }
    }

    fn pdf(&self, _: &math::Vec3, _: &math::Vec3, _: &math::Vec3) -> f32 {
        0.0
    }
}
