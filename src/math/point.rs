use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
};

use num_traits::{
    real::Real,
    sign::Signed,
};
use noisy_float::prelude::*;
use serde_derive::{Serialize, Deserialize};

use crate::math::{Num, Vector2, Vector3};

pub type Point2i = Vector2<u32>;
pub type Point2f = Vector2<N32>;
pub type Point3i = Vector3<u32>;
pub type Point3f = Vector3<N32>;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2<N: Num>(Vector2<N>);

impl_p2!();

impl<N: Num> Point2<N> {
    #[inline]
    pub fn new(x: N, y: N) -> Self {
        Point2(Vector2::new(x, y)) 
    }

    #[inline]
    pub fn x(&self) -> N { self.0.x() }

    #[inline]
    pub fn y(&self) -> N { self.0.y() }

    #[inline]
    pub fn set(&mut self, x: N, y: N) {
        self.0.set(x, y);
    }

    #[inline]
    pub fn dist_sq(&self, rhs: &Self) -> N {
        (rhs.0 - self.0).len_sq()
    }

    #[inline]
    pub fn permute(&self, x: usize, y: usize) -> Self {
        Point2(self.0.permute(x, y))
    }
}

impl <N: Num + Real> Point2<N> {
    #[inline]
    pub fn dist(&self, p: &Self) -> N {
        (p.0 - self.0).len()
    }

    #[inline]
    pub fn lerp(&self, rhs: &Self, t: N) -> Self {
        Point2(self.0 * (N::one() - t) + (rhs.0 * t))
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        Point2(self.0.ceil())
    }

    #[inline]
    pub fn floor(&self) -> Self {
        Point2(self.0.floor())
    }
}

impl <N: Num + Signed> Point2<N> {
    #[inline]
    pub fn abs(&self) -> Self {
        Point2(self.0.abs())
    }
}

impl<N: Num> From<Point3<N>> for Point2<N> {
    fn from(p: Point3<N>) -> Point2<N> {
        Point2(Vector2::new(p.x(), p.y()))
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3<N: Num>(Vector3<N>);

impl_p3!();

impl<N: Num> Point3<N> {
    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Point3(Vector3::new(x, y, z)) 
    }

    #[inline]
    pub fn x(&self) -> N { self.0.x() }

    #[inline]
    pub fn y(&self) -> N { self.0.y() }

    #[inline]
    pub fn z(&self) -> N { self.0.z() }

    #[inline]
    pub fn set(&mut self, x: N, y: N, z: N) {
        self.0.set(x, y, z);
    }

    #[inline]
    pub fn dist_sq(&self, rhs: &Self) -> N {
        (rhs.0 - self.0).len_sq()
    }

    #[inline]
    pub fn min(&self, rhs: &Self) -> Self {
        Point3(self.0.min(rhs.0))
    }

    #[inline]
    pub fn max(&self, rhs: &Self) -> Self {
        Point3(self.0.max(rhs.0))
    }

    #[inline]
    pub fn permute(&self, x: usize, y: usize, z: usize) -> Self {
        Point3(self.0.permute(x, y, z))
    }
}

impl <N: Num + Real> Point3<N> {
    #[inline]
    pub fn dist(&self, p: &Self) -> N {
        (p.0 - self.0).len()
    }

    #[inline]
    pub fn lerp(&self, rhs: &Self, t: N) -> Self {
        Point3(self.0 * (N::one() - t) + (rhs.0 * t))
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        Point3(self.0.ceil())
    }

    #[inline]
    pub fn floor(&self) -> Self {
        Point3(self.0.floor())
    }
}

impl <N: Num + Signed> Point3<N> {
    #[inline]
    pub fn abs(&self) -> Self {
        Point3(self.0.abs())
    }
}
