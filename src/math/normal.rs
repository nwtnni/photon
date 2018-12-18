use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Index, IndexMut,
    Neg,
};

use serde_derive::{Serialize, Deserialize};
use noisy_float::prelude::*;
use num_traits::Float;

use crate::math::{Num, Vec3};

pub type Normal3f = Normal3<N32>;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Normal3<N>(Vec3<N>);

impl<N> Normal3<N> {
    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Normal3(Vec3::new(x, y, z))
    }

    #[inline]
    pub fn set<V: Into<Self>>(&mut self, n: V) {
        *self = n.into();
    }
}

impl<N: Num> Normal3<N> {

    #[inline]
    pub fn x(&self) -> N { self.0.x() }

    #[inline]
    pub fn y(&self) -> N { self.0.y() }

    #[inline]
    pub fn z(&self) -> N { self.0.z() }

    #[inline]
    pub fn len_sq(&self) -> N {
        self.dot_n(self)
    }

    #[inline]
    pub fn dot_n(&self, rhs: &Self) -> N {
        self.x() * rhs.x() +
        self.y() * rhs.y() +
        self.z() * rhs.z()
    }

    #[inline]
    pub fn dot_v(&self, v: &Vec3<N>) -> N {
        self.dot_n(&Normal3::from(*v))
    }
}

impl<N: Num + Float> Normal3<N> {
    #[inline]
    pub fn normalize(&self) -> Self {
        self / self.len()
    }

    #[inline]
    pub fn len(&self) -> N {
        self.0.len()
    }

    #[inline]
    pub fn face_n(&self, n: &Normal3<N>) -> Self {
        if self.dot_n(n) < N::zero() { -self } else { *self }
    }

    #[inline]
    pub fn face_v(&self, v: &Vec3<N>) -> Self {
        if self.dot_v(v) < N::zero() { -self } else { *self }
    }

    #[inline]
    pub fn cross_v(&self, v: &Vec3<N>) -> Vec3<N> {
        self.0.cross_v(v)
    }
}

impl<N> From<Vec3<N>> for Normal3<N> {
    #[inline]
    fn from(v: Vec3<N>) -> Self { Normal3(v) }
}

impl<N> From<Normal3<N>> for Vec3<N> {
    #[inline]
    fn from(n: Normal3<N>) -> Self { n.0 }
}

impl<N> Index<usize> for Normal3<N> {
    type Output = N;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        self.0.index(i)
    }
}

impl<N> IndexMut<usize> for Normal3<N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.0.index_mut(i)
    }
}

impl<N: Num + Neg<Output = N>> Neg for Normal3<N> {
    type Output = Normal3<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Normal3::new(-self.x(), -self.y(), -self.z())
    }
}

impl<N: Num + Neg<Output = N>> Neg for &Normal3<N> {
    type Output = Normal3<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Normal3::new(-self.x(), -self.y(), -self.z())
    }
}

impl<N, M> From<[M; 3]> for Normal3<N>
    where M: Clone + Into<N>
{
    #[inline]
    fn from(v: [M; 3]) -> Normal3<N> {
        Normal3(Vec3::from(v))
    }
}

impl<N, X, Y, Z> From<(X, Y, Z)> for Normal3<N>
    where X: Into<N>,
          Y: Into<N>,
          Z: Into<N>,
{
    #[inline]
    fn from(n: (X, Y, Z)) -> Normal3<N> {
        Normal3(Vec3::from(n))        
    }
}

impl_all!(impl_add_v3v, Normal3<N>, Normal3<N>);
impl_mut!(impl_add_assign_v3v, Normal3<N>, Normal3<N>);

impl_all!(impl_sub_v3v, Normal3<N>, Normal3<N>);
impl_mut!(impl_sub_assign_v3v, Normal3<N>, Normal3<N>);

impl_all!(impl_mul_v3s, Normal3<N>, N);
impl_mut!(impl_mul_assign_v3s, Normal3<N>, N);

impl_all!(impl_div_v3s, Normal3<N>, N);
impl_mut!(impl_div_assign_v3s, Normal3<N>, N);
