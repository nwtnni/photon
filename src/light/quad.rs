use crate::geom;
use crate::light;
use crate::math;

impl<'scene> light::Light for geom::Quad<'scene> {
    fn eval(&self, ray: &math::Ray) -> math::Vec3 {
        if ray.d.dot(&self.n) < 0.0 {
            self.emit.expect("Must be emitter to be in scene as a light")
        } else {
            math::Vec3::default()
        }
    }

    fn sample(&self, p: &math::Vec3) -> light::Sample {
        let l = self.p
            + self.u * rand::random::<f32>()
            + self.v * rand::random::<f32>();
        light::Sample {
            d: (l - p).normalize(),
            t: (l - p).len(),
            a: (l - p).normalize().dot(&self.n).abs() / (l - p).len_sq(),
            p: 1.0 / (self.u.len() * self.v.len()),
        }
    }

    fn pdf(&self, _: &math::Ray) -> f32 {
        1.0 / (self.u.len() * self.v.len())
    }

    fn downcast_point(&self) -> Option<light::Point> {
        None
    }
}
