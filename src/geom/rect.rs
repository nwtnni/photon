use crate::bxdf;
use crate::geom;
use crate::light;
use crate::math;

#[readonly::make]
#[derive(Copy, Clone, Debug)]
pub struct Rect<'scene> {
    pub p: math::Vec3,
    pub u: math::Vec3,
    pub v: math::Vec3,
    pub n: math::Vec3,
    pub bound: geom::Box3,
    pub bxdf: &'scene dyn bxdf::BxDF,
    pub emit: Option<math::Vec3>,
}

impl<'scene> Rect<'scene> {
    pub fn new(
        p: math::Vec3,
        u: math::Vec3,
        v: math::Vec3,
        bxdf: &'scene dyn bxdf::BxDF,
        emit: Option<math::Vec3>
    ) -> Self {
        Rect {
            p, u, v,
            n: u.cross(&v).normalize(),
            bound: geom::Box3::new(p, p + u).union_v(&(p + v)),     
            bxdf,
            emit,
        }
    }

    pub fn as_light(&self) -> &dyn light::Light {
        self
    }
}

impl<'scene> geom::Surface<'scene> for Rect<'scene> {

    fn bound(&self) -> geom::Box3 {
        self.bound
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        let t = (self.p - ray.p).dot(&self.n) / ray.d.dot(&self.n);
        let p = ray.at(t) - self.p;

        let u = p.dot(&self.u) / self.u.len_sq();
        if u < 0.0 || u > 1.0 { return false }

        let v = p.dot(&self.v) / self.v.len_sq();
        if v < 0.0 || v > 1.0 { return false }

        hit.t = t;
        hit.p = ray.at(t);
        hit.n = if ray.d.dot(&self.n) < 0.0 { self.n } else { -self.n };
        hit.u = u;
        hit.v = v;
        hit.bxdf = Some(self.bxdf);

        true
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        let t = (self.p - ray.p).dot(&self.n) / ray.d.dot(&self.n);
        let p = ray.at(t) - self.p;

        let u = p.dot(&self.u) / self.u.len_sq();
        if u < 0.0 || u > 1.0 { return false }

        let v = p.dot(&self.v) / self.v.len_sq();
        if v < 0.0 || v > 1.0 { return false }

        true
    }
}
