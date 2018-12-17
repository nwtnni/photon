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
pub type Point3i = Point3<i32>;
pub type Point3f = Point3<N32>;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2<N>(Vec2<N>);

impl<N> Point2<N> {
    #[inline]
    pub fn new(x: N, y: N) -> Self {
        Point2(Vec2::new(x, y))
    }

    #[inline]
    pub fn set<V: Into<Self>>(&mut self, v: V) {
        *self = v.into();
    }
}

impl<N: Num> Point2<N> {
    #[inline]
    pub fn broadcast(v: N) -> Self {
        Point2(Vec2::broadcast(v))
    }

    #[inline]
    pub fn x(&self) -> N { self.0.x() } 

    #[inline]
    pub fn y(&self) -> N { self.0.y() }

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

impl <N: Num + Signed> Point2<N> {
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

impl<N, M> From<[M; 2]> for Point2<N>
    where M: Clone + Into<N>
{
    #[inline]
    fn from(v: [M; 2]) -> Point2<N> {
        Point2(Vec2::from(v))
    }
}

impl<N, X, Y> From<(X, Y)> for Point2<N>
    where X: Into<N>,
          Y: Into<N>,
{
    #[inline]
    fn from(p: (X, Y)) -> Point2<N> {
        Point2(Vec2::from(p))        
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

impl<N> Point3<N> {
    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Point3(Vec3::new(x, y, z)) 
    }

    #[inline]
    pub fn set<P: Into<Self>>(&mut self, p: P) {
        *self = p.into();
    }
}

impl<N: Num> Point3<N> {
    #[inline]
    pub fn broadcast(v: N) -> Self {
        Point3(Vec3::broadcast(v)) 
    }

    #[inline]
    pub fn x(&self) -> N { self.0.x() }

    #[inline]
    pub fn y(&self) -> N { self.0.y() }

    #[inline]
    pub fn z(&self) -> N { self.0.z() }

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

impl <N: Num + Signed> Point3<N> {
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

impl<N, M> From<[M; 3]> for Point3<N>
    where M: Clone + Into<N>
{
    #[inline]
    fn from(v: [M; 3]) -> Point3<N> {
        Point3(Vec3::from(v))
    }
}

impl<N, X, Y, Z> From<(X, Y, Z)> for Point3<N>
    where X: Into<N>,
          Y: Into<N>,
          Z: Into<N>,
{
    #[inline]
    fn from(p: (X, Y, Z)) -> Point3<N> {
        Point3(Vec3::from(p))        
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
