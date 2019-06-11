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
        let cos_i = d.dot(n);

        let (cos_i, eta, n) = if cos_i <= 0.0 {
            (-cos_i, 1.0 / self.eta, -n)
        } else {
            (cos_i, self.eta, *n)
        };

        let (reflect, cos_t) = bxdf::dieletric(cos_i, eta);

        if rand::random::<f32>() < reflect {
            bxdf::Sample {
                d: math::reflect(*d, n).normalize(),
                v: math::Vec3::broadcast(reflect / cos_i),
                p: reflect,
                delta: true,
            }
        } else {
            bxdf::Sample {
                d: ((n * cos_i - d) * (1.0 / eta) + (n * -cos_t)).normalize(),
                v: math::Vec3::broadcast((1.0 - reflect) / cos_t),
                p: 1.0 - reflect,
                delta: true,
            }
        }
    }

    fn pdf(&self, _: &math::Vec3, _: &math::Vec3, _: &math::Vec3) -> f32 {
        0.0
    }
}
