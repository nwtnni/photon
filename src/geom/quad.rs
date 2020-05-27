use crate::bxdf;
use crate::geom;
use crate::light;
use crate::math;

#[readonly::make]
#[derive(Copy, Clone, Debug)]
pub struct Quad<'scene> {
    pub p: math::Vec3,
    pub u: math::Vec3,
    pub v: math::Vec3,
    pub n: math::Vec3,
    pub bound: geom::Box3,
    pub bxdf: &'scene bxdf::Any<'scene>,
    pub emit: Option<math::Vec3>,
}

impl<'scene> Quad<'scene> {
    pub fn new(
        p: math::Vec3,
        u: math::Vec3,
        v: math::Vec3,
        bxdf: &'scene bxdf::Any<'scene>,
        emit: Option<math::Vec3>
    ) -> Self {
        Quad {
            p, u, v,
            n: u.cross(&v).normalize(),
            bound: geom::Box3::new(p, p + u + v)
                .union_v(&(p + u))
                .union_v(&(p + v)),
            bxdf,
            emit,
        }
    }

    pub fn as_light(&self) -> &dyn light::Light {
        self
    }
}

impl<'scene> geom::Surface<'scene> for Quad<'scene> {

    fn bound(&self) -> geom::Box3 {
        self.bound
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Hit<'scene>) -> bool {

        const EPSILON: f32 = 0.0000001;

        let h = ray.d.cross(&self.v);
        let det = self.u.dot(&h);

        if det > -EPSILON && det < EPSILON { return false }

        let inv = 1.0 / det;
        let s = ray.p - self.p;
        let u = inv * s.dot(&h);
        if u < 0.0 || u > 1.0 { return false }

        let q = s.cross(&self.u);
        let v = inv * ray.d.dot(&q);
        if v < 0.0 || v > 1.0 { return false }

        let t = inv * self.v.dot(&q);
        if t < ray.min || t > ray.max { return false }

        ray.set_max(t);
        hit.t = t;
        hit.p = ray.at(t);
        hit.n = if ray.d.dot(&self.n) < 0.0 { self.n } else { -self.n };
        hit.u = u;
        hit.v = v;
        hit.bxdf = Some(self.bxdf);
        hit.emit = if ray.d.dot(&self.n) < 0.0 { self.emit } else { None };

        true
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {

        const EPSILON: f32 = 0.0000001;

        let h = ray.d.cross(&self.v);
        let det = self.u.dot(&h);

        if det > -EPSILON && det < EPSILON { return false }

        let inv = 1.0 / det;
        let s = ray.p - self.p;
        let u = inv * s.dot(&h);
        if u < 0.0 || u > 1.0 { return false }

        let q = s.cross(&self.u);
        let v = inv * ray.d.dot(&q);
        if v < 0.0 || v > 1.0 { return false }

        let t = inv * self.v.dot(&q);
        if t < ray.min || t > ray.max { return false }

        true
    }
}
