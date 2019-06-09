use crate::bxdf;
use crate::geom;
use crate::math;

const MAX_STEPS: usize = 64;
const EPSILON: f32 = 0.000_01;
const DX: math::Vec3 = math::Vec3::new(EPSILON, 0.0, 0.0);
const DY: math::Vec3 = math::Vec3::new(0.0, EPSILON, 0.0);
const DZ: math::Vec3 = math::Vec3::new(0.0, 0.0, EPSILON);

#[derive(Debug)]
pub struct SDF<'scene> {
    bxdf: &'scene dyn bxdf::BxDF,
    field: Field,
}

impl<'scene> SDF<'scene> {
    pub fn new(bxdf: &'scene dyn bxdf::BxDF, field: Field) -> Self {
        SDF { bxdf, field }
    }
}

impl<'scene> geom::Surface<'scene> for SDF<'scene> {
    fn bound(&self) -> geom::Box3 {
        self.field.bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        if self.field.hit(ray, hit) {
            hit.bxdf = Some(self.bxdf);
            true
        } else {
            false
        }
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.field.hit_any(ray)
    }
}

pub struct Field {
    field: Box<dyn Fn(&math::Vec3) -> f32 + Send + Sync>,
    bound: geom::Box3, 
}

impl Field {
    pub fn at(&self, point: &math::Vec3) -> f32 {
        (self.field)(&point) 
    }

    pub fn normal(&self, point: &math::Vec3) -> math::Vec3 {
        let dx = self.at(&(point + DX)) - self.at(&(point - DX));
        let dy = self.at(&(point + DY)) - self.at(&(point - DY));
        let dz = self.at(&(point + DZ)) - self.at(&(point - DZ));
        math::Vec3::new(dx, dy, dz).normalize()
    }

    pub fn sphere(radius: f32) -> Self {
        let c = math::Vec3::default();
        let r = math::Vec3::broadcast(radius);
        Field {
            field: Box::new(move |point| point.len() - radius),
            bound: geom::Box3::new(c - r, c + r),
        }
    }

    pub fn cube(side: f32) -> Self {
        let a = math::Vec3::default();
        let b = math::Vec3::broadcast(side);
        Field {
            bound: geom::Box3::new(a, b),
            field: Box::new(move |point| {
                let d = point.abs() - b;
                d.max(&a).len() + math::min(0.0, math::max(d.x(), math::max(d.y(), d.z())))
            }),
        }
    }

    pub fn union(self, rhs: Field) -> Self {
        Field {
            bound: self.bound.union_b(&rhs.bound),
            field: Box::new(move |point| math::min(self.at(point), rhs.at(point))),
        }
    }

    pub fn intersect(self, rhs: Field) -> Self {
        Field {
            bound: self.bound.intersect(&rhs.bound),
            field: Box::new(move |point| math::max(self.at(point), rhs.at(point))),
        }
    }

    pub fn subtract(self, rhs: Field) -> Self {
        Field {
            bound: self.bound,
            field: Box::new(move |point| math::max(self.at(point), -rhs.at(point))),
        }
    }
}

impl<'scene> geom::Surface<'scene> for Field {
    fn bound(&self) -> geom::Box3 {
        self.bound
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        let mut t = 0.0;
        for _ in 0..MAX_STEPS {
            let p = ray.origin + ray.dir * t;
            let dt = self.at(&p);
            if dt < EPSILON {
                ray.set_max(t);
                hit.t = t;
                hit.p = ray.at(t);
                hit.n = self.normal(&p);
                return true
            }
            t += dt;
        }
        false
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        let mut t = 0.0;
        for _ in 0..MAX_STEPS {
            let dt = self.at(&(ray.origin + ray.dir * t));
            if dt < EPSILON { return true }
            t += dt;
        }
        false
    }
}

impl std::ops::BitAnd for Field {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersect(rhs)
    }
}

impl std::ops::BitOr for Field {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl std::ops::Sub for Field {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(rhs)
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "<ABSTRACT>")
    }
}
