use std::ops::Index;

use noisy_float::prelude::*;
use num_traits::real::Real;

use crate::math::{Num, Point2i,Point2, Point3, Vec2, Vec3};

pub type Bounds2i = Bounds2<i32>;
pub type Bounds2f = Bounds2<N32>;
pub type Bounds3i = Bounds3<i32>;
pub type Bounds3f = Bounds3<N32>;

#[derive(Copy, Clone, Default, Debug)]
pub struct Bounds2<N> {
    min: Point2<N>,
    max: Point2<N>,
}

impl<N: Num> Bounds2<N> {

    #[inline]
    pub fn new(p: Point2<N>) -> Self {
        Bounds2 {
            min: p,
            max: p,
        }
    }

    pub fn corner(&self, i: usize) -> Point2<N> {
        let x = if i & 1 == 0 { self.min.x() } else { self.max.x() };
        let y = if i & 2 == 0 { self.min.y() } else { self.max.y() };
        Point2::new(x, y)
    }

    pub fn union_p(&self, p: &Point2<N>) -> Self {
        Bounds2 {
            min: self.min.min(*p),
            max: self.max.max(*p),
        }
    }

    pub fn union_b(&self, b: &Self) -> Self {
        Bounds2 {
            min: self.min.min(b.min),
            max: self.max.max(b.max),
        }
    }

    pub fn intersect(&self, b: &Self) -> Self {
        Bounds2 {
            min: self.min.max(b.min),
            max: self.max.min(b.max),
        }
    }

    pub fn overlaps(&self, b: &Self) -> bool {
        let x = self.max.x() >= b.min.x() && self.min.x() <= b.max.x();
        let y = self.max.y() >= b.min.y() && self.min.y() <= b.max.y();
        x && y
    }

    pub fn inside(&self, p: Point2<N>) -> bool {
        let x = p.x() >= self.min.x() && p.x() <= self.max.x();
        let y = p.y() >= self.min.y() && p.y() <= self.max.y();
        x && y
    }

    pub fn inside_ex(&self, p: Point2<N>) -> bool {
        let x = p.x() >= self.min.x() && p.x() < self.max.x();
        let y = p.y() >= self.min.y() && p.y() < self.max.y();
        x && y
    }

    pub fn expand(&self, d: N) -> Self {
        let delta = Vec2::broadcast(d);
        Bounds2 {
            min: self.min - delta,
            max: self.max + delta,
        }
    }

    pub fn diagonal(&self) -> Vec2<N> {
        self.max - self.min
    }

    pub fn area(&self) -> N {
        let d = self.diagonal();
        d.x() * d.y()
    }

    pub fn max_extent(&self) -> usize {
        self.diagonal().max_dim()
    }
}

impl<N: Num + Real> Bounds2<N> {
    pub fn lerp(&self, t: N) -> Point2<N> {
        self.min.lerp(&self.max, t)
    }

    pub fn offset(&self, p: &Point2<N>) -> Vec2<N> {
        let d = self.diagonal();
        let mut o = p - self.min;
        o[0] /= d.x();
        o[1] /= d.y();
        o
    }

    pub fn bounding_circle(&self) -> (Point2<N>, N) {
        let center = (self.min + self.max) / N::two();
        let radius = center.dist(&self.max);
        (center, radius)
    }
}

impl<N> Index<usize> for Bounds2<N> {
    type Output = Point2<N>;
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
        | 0 => &self.min,
        | 1 => &self.max,
        | n => panic!("[INTERNAL ERROR]: invalid index {}", n),
        }
    }
}

impl Bounds2i {
    fn iter(&self) -> impl Iterator<Item = Point2i> {
        let min_x = self.min.x();
        let min_y = self.min.y();
        let max_x = self.max.x();
        let max_y = self.max.y();
        (min_y..max_y).flat_map(move |y| {
            (min_x..max_x).map(move |x| {
                Point2i::new(x, y)
            })
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Bounds3<N> {
    min: Point3<N>,
    max: Point3<N>,
}

impl<N: Num> Default for Bounds3<N> {
    #[inline]
    fn default() -> Self {
        let min = Point3::broadcast(N::max_value());
        let max = Point3::broadcast(N::min_value());
        Bounds3 { min, max }
    }
}

impl<N: Num> From<Point3<N>> for Bounds3<N> {
    #[inline]
    fn from(p: Point3<N>) -> Self {
        Bounds3 { min: p, max: p }
    }
}

impl<N: Num> Bounds3<N> {

    #[inline]
    pub fn new(p1: Point3<N>, p2: Point3<N>) -> Self {
        Bounds3 {
            min: p1.min(p2),
            max: p2.max(p2),
        }
    }

    pub fn corner(&self, i: usize) -> Point3<N> {
        let x = if i & 1 == 0 { self.min.x() } else { self.max.x() };
        let y = if i & 2 == 0 { self.min.y() } else { self.max.y() };
        let z = if i & 4 == 0 { self.min.z() } else { self.max.z() };
        Point3::new(x, y, z)
    }

    pub fn union_p(&self, p: &Point3<N>) -> Self {
        Bounds3 {
            min: self.min.min(*p),
            max: self.max.max(*p),
        }
    }

    pub fn union_b(&self, b: &Self) -> Self {
        Bounds3 {
            min: self.min.min(b.min),
            max: self.max.max(b.max),
        }
    }

    pub fn intersect(&self, b: &Self) -> Self {
        Bounds3 {
            min: self.min.max(b.min),
            max: self.max.min(b.max),
        }
    }

    pub fn overlaps(&self, b: &Self) -> bool {
        let x = self.max.x() >= b.min.x() && self.min.x() <= b.max.x();
        let y = self.max.y() >= b.min.y() && self.min.y() <= b.max.y();
        let z = self.max.z() >= b.min.z() && self.min.z() <= b.max.z();
        x && y && z
    }

    pub fn inside(&self, p: Point3<N>) -> bool {
        let x = p.x() >= self.min.x() && p.x() <= self.max.x();
        let y = p.y() >= self.min.y() && p.y() <= self.max.y();
        let z = p.z() >= self.min.z() && p.z() <= self.max.z();
        x && y && z
    }

    pub fn inside_ex(&self, p: Point3<N>) -> bool {
        let x = p.x() >= self.min.x() && p.x() < self.max.x();
        let y = p.y() >= self.min.y() && p.y() < self.max.y();
        let z = p.z() >= self.min.z() && p.z() < self.max.z();
        x && y && z
    }

    pub fn expand(&self, d: N) -> Self {
        let delta = Vec3::broadcast(d);
        Bounds3 {
            min: self.min - delta,
            max: self.max + delta,
        }
    }

    pub fn diagonal(&self) -> Vec3<N> {
        self.max - self.min
    }

    pub fn area(&self) -> N {
        let d = self.diagonal();
        (N::one() + N::one()) *
        (d.x() * d.y() + d.x() * d.z() + d.y() * d.z())
    }

    pub fn volume(&self) -> N {
        let d = self.diagonal();
        d.x() * d.y() * d.z()
    }

    pub fn max_extent(&self) -> usize {
        self.diagonal().max_dim()
    }
}

impl<N: Num + Real> Bounds3<N> {
    pub fn lerp(&self, t: N) -> Point3<N> {
        self.min.lerp(&self.max, t)
    }

    pub fn offset(&self, p: &Point3<N>) -> Vec3<N> {
        let d = self.diagonal();
        let mut o = p - self.min;
        o[0] /= d.x();
        o[1] /= d.y();
        o[2] /= d.z();
        o
    }

    pub fn bounding_sphere(&self) -> (Point3<N>, N) {
        let center = (self.min + self.max) / N::two();
        let radius = center.dist(&self.max);
        (center, radius)
    }
}

impl<N> Index<usize> for Bounds3<N> {
    type Output = Point3<N>;
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
        | 0 => &self.min,
        | 1 => &self.max,
        | n => panic!("[INTERNAL ERROR]: invalid index {}", n),
        }
    }
}
