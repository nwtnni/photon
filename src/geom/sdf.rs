use crate::bxdf;
use crate::geom;
use crate::math;

const HORIZON: f32 = 100.0;
const EPSILON: f32 = 0.001;
const DX: math::Vec3 = math::Vec3::new(EPSILON, 0.0, 0.0);
const DY: math::Vec3 = math::Vec3::new(0.0, EPSILON, 0.0);
const DZ: math::Vec3 = math::Vec3::new(0.0, 0.0, EPSILON);

#[derive(Debug)]
pub struct SDF<'scene> {
    bxdf: &'scene dyn bxdf::BxDF,
    shape: Shape,
}

impl<'scene> SDF<'scene> {
    pub fn new(bxdf: &'scene dyn bxdf::BxDF, shape: Shape) -> Self {
        SDF { bxdf, shape }
    }
}

impl<'scene> geom::Surface<'scene> for SDF<'scene> {
    fn bound(&self) -> geom::Box3 {
        self.shape.bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        if self.shape.hit(ray, hit) {
            hit.bxdf = Some(self.bxdf);
            hit.emit = None;
            true
        } else {
            false
        }
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.shape.hit_any(ray)
    }
}

#[derive(Clone, Debug)]
pub struct Shape {
    bound: geom::Box3,
    shape: Tree,
}

impl Shape {
    pub fn sphere(radius: f32) -> Self {
        let c = math::Vec3::default();
        let r = math::Vec3::broadcast(radius);
        Shape {
            bound: geom::Box3::new(c - r, c + r),
            shape: Tree::Sphere(radius),
        }
    }

    pub fn cube(side: f32) -> Self {
        let a = math::Vec3::broadcast(-side);
        let b = math::Vec3::broadcast(side);
        Shape {
            bound: geom::Box3::new(a, b),
            shape: Tree::Box(b, 0.0),
        }
    }

    pub fn sharp_box(corner: math::Vec3) -> Self {
        Shape {
            bound: geom::Box3::new(-corner, corner),
            shape: Tree::Box(corner, 0.0),
        }
    }

    pub fn round_box(corner: math::Vec3, radius: f32) -> Self {
        Shape {
            bound: geom::Box3::new(-corner, corner),
            shape: Tree::Box(corner, radius),
        }
    }

    pub fn union(self, rhs: Shape) -> Self {
        Shape {
            bound: self.bound.union_b(&rhs.bound),
            shape: Tree::Union(Box::new(self.shape), Box::new(rhs.shape)),
        }
    }

    pub fn intersect(self, rhs: Shape) -> Self {
        Shape {
            bound: self.bound.intersect(&rhs.bound),
            shape: Tree::Intersect(Box::new(self.shape), Box::new(rhs.shape)),
        }
    }

    pub fn subtract(self, rhs: Shape) -> Self {
        Shape {
            bound: self.bound,
            shape: Tree::Subtract(Box::new(self.shape), Box::new(rhs.shape)),
        }
    }

    pub fn smooth_union(self, rhs: Shape, k: f32) -> Self {
        Shape {
            bound: self.bound.union_b(&rhs.bound),
            shape: Tree::SmoothUnion(Box::new(self.shape), Box::new(rhs.shape), k),
        }
    }

    pub fn smooth_intersect(self, rhs: Shape, k: f32) -> Self {
        Shape {
            bound: self.bound.intersect(&rhs.bound),
            shape: Tree::SmoothIntersect(Box::new(self.shape), Box::new(rhs.shape), k),
        }
    }

    pub fn smooth_subtract(self, rhs: Shape, k: f32) -> Self {
        Shape {
            bound: self.bound,
            shape: Tree::SmoothSubtract(Box::new(self.shape), Box::new(rhs.shape), k),
        }
    }

    pub fn scale(self, c: f32) -> Self {
        Shape {
            bound: self.bound.scale(c),
            shape: Tree::Scale(Box::new(self.shape), c),
        }
    }

    pub fn translate(self, v: math::Vec3) -> Self {
        Shape {
            bound: self.bound.translate(&v),
            shape: Tree::Translate(Box::new(self.shape), v),
        }
    }
}

impl<'scene> geom::Surface<'scene> for Shape {
    fn bound(&self) -> geom::Box3 {
        self.bound
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        self.shape.hit(ray, hit)
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.shape.hit_any(ray)
    }
}

impl std::ops::BitAnd for Shape {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersect(rhs)
    }
}

impl std::ops::BitOr for Shape {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl std::ops::Sub for Shape {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(rhs)
    }
}

#[derive(Clone, Debug)]
enum Tree {
    Box(math::Vec3, f32),
    Sphere(f32),
    Union(Box<Tree>, Box<Tree>), 
    Intersect(Box<Tree>, Box<Tree>),
    Subtract(Box<Tree>, Box<Tree>),
    SmoothUnion(Box<Tree>, Box<Tree>, f32),
    SmoothIntersect(Box<Tree>, Box<Tree>, f32),
    SmoothSubtract(Box<Tree>, Box<Tree>, f32),
    Scale(Box<Tree>, f32),
    Translate(Box<Tree>, math::Vec3),
}

impl Tree {
    pub fn at(&self, point: &math::Vec3) -> f32 {
        match self {
        | Tree::Box(corner, radius) => {
            let d = point.abs() - corner; 
            let a = d.max(&math::Vec3::default()).len() - radius;
            let b = math::min(0.0, math::max(d.x(), math::max(d.y(), d.z())));
            a + b
        }
        | Tree::Sphere(radius) => {
            point.len() - radius
        }
        | Tree::Union(lhs, rhs) => {
            math::min(lhs.at(point), rhs.at(point))
        }
        | Tree::Intersect(lhs, rhs) => {
            math::max(lhs.at(point), rhs.at(point))
        }
        | Tree::Subtract(lhs, rhs) => {
            math::max(lhs.at(point), -rhs.at(point))
        }
        | Tree::SmoothUnion(lhs, rhs, k) => {
            let a = lhs.at(point);
            let b = rhs.at(point);
            let h = math::max(k - (a - b).abs(), 0.0) / k;
            math::min(a, b) - h * h * k * 0.25
        }
        | Tree::SmoothIntersect(lhs, rhs, k) => {
            let a = lhs.at(point);
            let b = rhs.at(point);
            let h = math::max(k - (a - b).abs(), 0.0) / k;
            math::max(a, b) + h * h * k * 0.25
        }
        | Tree::SmoothSubtract(lhs, rhs, k) => {
            let a = lhs.at(point);
            let b = -rhs.at(point);
            let h = math::max(k - (a - b).abs(), 0.0) / k;
            math::max(a, b) + h * h * k * 0.25
        }
        | Tree::Scale(shape, scale) => {
            shape.at(&(point / scale)) * scale
        }
        | Tree::Translate(shape, offset) => {
            shape.at(&(point - offset))
        }
        }
    }

    pub fn normal(&self, point: &math::Vec3) -> math::Vec3 {
        let dx = self.at(&(point + DX)) - self.at(&(point - DX));
        let dy = self.at(&(point + DY)) - self.at(&(point - DY));
        let dz = self.at(&(point + DZ)) - self.at(&(point - DZ));
        math::Vec3::new(dx, dy, dz).normalize()
    }
}

impl<'scene> geom::Surface<'scene> for Tree {
    fn bound(&self) -> geom::Box3 {
        panic!("Bound should be stored in parent Shape")
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        let mut t = ray.min;
        while t < HORIZON && t < ray.max {
            let p = ray.p + ray.d * t;
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
        let mut t = ray.min;
        while t < HORIZON && t < ray.max {
            let dt = self.at(&(ray.p + ray.d * t));
            if dt < EPSILON { return true }
            t += dt;
        }
        false
    }
}
