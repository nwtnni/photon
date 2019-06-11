use crate::geom;
use crate::light;
use crate::math;

impl<'scene> light::Light for geom::Rect<'scene> {
    fn intensity(&self) -> math::Vec3 {
        self.emit.expect("Must be emitter to be in scene as a light");
    }

    fn sample(&self, p: &math::Vec3, r: &mut light::Record) {
        let l = self.p
            + self.u * rand::random::<f32>()
            + self.v * rand::random::<f32>();
        let wi = l - p;
        r.d = wi.normalize();
        r.a = math::max(0.0, r.d.dot(&self.n)) / wi.len_sq();
        r.p = 1.0 / (self.u.len() * self.v.len());
        r.t = wi.len();
    }

    fn pdf(&self, _: &math::Ray) -> f32 {
        1.0 / (self.u.len() * self.v.len())
    }

    fn downcast_point(&self) -> Option<light::Point> {
        None
    }
}
