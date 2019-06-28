use crate::bxdf;
use crate::math;

#[readonly::make]
#[derive(Copy, Clone, Debug, Default)]
pub struct Fresnel {
    pub d: math::Vec3,
    pub n: math::Vec3,
    pub eta: f32,
    pub cos_i: f32,
    pub cos_t: f32,
    pub reflect: f32,
}

impl Fresnel {
    pub fn dieletric(d: &math::Vec3, n: &math::Vec3, eta: f32) -> Self {

        let cos_i = d.dot(n);
        
        let (n, eta_i, eta_t) = if cos_i <= 0.0 {
            (-n, eta, 1.0)
        } else {
            (*n, 1.0, eta)
        };

        let cos_i = cos_i.abs();
        let cos_t_sq = 1.0 - eta_i.powi(2) * (1.0 - cos_i.powi(2)) / eta_t.powi(2);

        if cos_t_sq < 0.0 {
            return Fresnel { d: *d, n, cos_i, reflect: 1.0, ..Default::default() }
        }

        let eta = eta_t / eta_i;
        let cos_t = cos_t_sq.sqrt();

        let par = (eta * cos_i - cos_t) / (eta * cos_i + cos_t); 
        let per = (cos_i - eta * cos_t) / (cos_i + eta * cos_t);
        let reflect = 0.5 * (par.powi(2) + per.powi(2));

        Fresnel {
            d: *d,
            n,
            cos_i,
            cos_t,
            eta,
            reflect
        }
    }

    pub fn mirror(d: &math::Vec3, n: &math::Vec3) -> Self {
        Fresnel {
            d: *d,
            n: *n,
            cos_i: d.dot(n).abs(),
            reflect: 1.0,
            .. Default::default()
        }
    }

    pub fn reflect(&self) -> bxdf::Sample {
        let d = (self.n * 2.0 * self.cos_i - self.d).normalize();
        let v = math::Vec3::broadcast(self.reflect / self.cos_i);
        let p = self.reflect;
        bxdf::Sample { d, v, p, delta: true }
    }

    pub fn refract(&self) -> bxdf::Sample {
        let d = (self.n * self.cos_i - self.d) / self.eta - self.n * self.cos_t;
        let v = math::Vec3::broadcast((1.0 - self.reflect) / self.cos_t);
        let p = 1.0 - self.reflect;
        bxdf::Sample { d, v, p, delta: true }
    }
}
