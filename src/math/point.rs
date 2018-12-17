use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Index, IndexMut,
};

use num_traits::{
    real::Real,
    sign::Signed,
};
use noisy_float::prelude::*;
use serde_derive::{Serialize, Deserialize};

use crate::math::{Num, Vec2, Vec3};

pub type Point2i = Point2<i32>;
pub type Point2f = Point2<N32>;
pub type Point2d = Point2<N64>;
pub type Point3i = Point3<i32>;
pub type Point3f = Point3<N32>;
pub type Point3d = Point3<N64>;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2<N>(Vec2<N>);

impl<N: Num> Point2<N> {

    #[inline]
    pub fn unpack((x, y): (N, N)) -> Self {
        Self::new(x, y)
    }

    #[inline]
    pub fn pack(&mut self, (x, y): (N, N)) {
        self.set(x, y)
    }

    #[inline]
    pub fn new(x: N, y: N) -> Self {
        Point2(Vec2::new(x, y))
    }

    #[inline]
    pub fn fill(v: N) -> Self {
        Point2(Vec2::fill(v))
    }

    #[inline]
    pub fn x(&self) -> N { self.0.x() } 

    #[inline]
    pub fn y(&self) -> N { self.0.y() }

    #[inline]
    pub fn set(&mut self, x: N, y: N) {
        self.0.set(x, y)
    }

    #[inline]
    pub fn min(&self, rhs: &Self) -> Self {
        Point2(self.0.min(rhs.0))
    }

    #[inline]
    pub fn max(&self, rhs: &Self) -> Self {
        Point2(self.0.max(rhs.0))
    }

    #[inline]
    pub fn permute(&self, x: usize, y: usize) -> Self {
        Point2(self.0.permute(x, y))
    }

    #[inline]
    pub fn dist_sq(&self, rhs: &Self) -> N {
        (rhs.0 - self.0).len_sq()
    }
}

impl <N: Num + Real> Point2<N> {
    #[inline]
    pub fn dist(&self, p: &Self) -> N {
        (p.0 - self.0).len()
    }

    #[inline]
    pub fn lerp(&self, rhs: &Self, t: N) -> Self {
        self * (N::one() - t) + (rhs * t)
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

impl <N: Signed> Point2<N> {
    #[inline]
    pub fn abs(&self) -> Self {
        Point2(self.0.abs())
    }
}

impl<N: Num> From<Point3<N>> for Point2<N> {
    #[inline]
    fn from(p: Point3<N>) -> Point2<N> {
        Point2(Vec2::new(p.x(), p.y()))
    }
}

impl<N> Index<usize> for Point2<N> {
    type Output = N;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        self.0.index(i)
    }
}

impl<N> IndexMut<usize> for Point2<N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.0.index_mut(i)
    }
}

impl_all!(impl_add_v2v, Point2<N>, Point2<N>);
impl_mut!(impl_add_assign_v2v, Point2<N>, Point2<N>);
impl_all!(impl_add_v2v, Point2<N>, Vec2<N>);
impl_mut!(impl_add_assign_v2v, Point2<N>, Vec2<N>);
impl_all!(impl_sub_v2v, Vec2<N>, Point2<N>, Point2<N>);
impl_all!(impl_sub_v2v, Point2<N>, Vec2<N>);
impl_mut!(impl_sub_assign_v2v, Point2<N>, Vec2<N>);

impl_all!(impl_mul_v2s, Point2<N>, N);
impl_mut!(impl_mul_assign_v2s, Point2<N>, N);
impl_all!(impl_div_v2s, Point2<N>, N);
impl_mut!(impl_div_assign_v2s, Point2<N>, N);

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3<N>(Vec3<N>);

impl<N: Num> Point3<N> {

    #[inline]
    pub fn unpack((x, y, z): (N, N, N)) -> Self {
        Self::new(x, y, z)
    }

    #[inline]
    pub fn pack(&mut self, (x, y, z): (N, N, N)) {
        self.set(x, y, z)
    }

    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Point3(Vec3::new(x, y, z)) 
    }

    #[inline]
    pub fn fill(v: N) -> Self {
        Point3(Vec3::fill(v)) 
    }

    #[inline]
    pub fn x(&self) -> N { self.0.x() }

    #[inline]
    pub fn y(&self) -> N { self.0.y() }

    #[inline]
    pub fn z(&self) -> N { self.0.z() }

    #[inline]
    pub fn set(&mut self, x: N, y: N, z: N) {
        self.0.set(x, y, z)
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
        self * (N::one() - t) + (rhs * t)
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

impl <N: Signed> Point3<N> {
    #[inline]
    pub fn abs(&self) -> Self {
        Point3(self.0.abs())
    }
}

impl<N> Index<usize> for Point3<N> {
    type Output = N;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        self.0.index(i)
    }
}

impl<N> IndexMut<usize> for Point3<N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.0.index_mut(i)
    }
}

impl_all!(impl_add_v3v, Point3<N>, Point3<N>);
impl_mut!(impl_add_assign_v3v, Point3<N>, Point3<N>);
impl_all!(impl_add_v3v, Point3<N>, Vec3<N>);
impl_mut!(impl_add_assign_v3v, Point3<N>, Vec3<N>);
impl_all!(impl_sub_v3v, Vec3<N>, Point3<N>, Point3<N>);
impl_all!(impl_sub_v3v, Point3<N>, Vec3<N>);
impl_mut!(impl_sub_assign_v3v, Point3<N>, Vec3<N>);

impl_all!(impl_mul_v3s, Point3<N>, N);
impl_mut!(impl_mul_assign_v3s, Point3<N>, N);
impl_all!(impl_div_v3s, Point3<N>, N);
impl_mut!(impl_div_assign_v3s, Point3<N>, N);
