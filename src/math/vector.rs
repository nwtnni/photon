#![allow(dead_code)]

use std::cmp;
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
use num_traits::{
    Float,
    sign::Signed,
};

use crate::math::{Num, Normal3};

pub type Vec2i = Vec2<u32>;
pub type Vec2f = Vec2<N32>;
pub type Vec3i = Vec3<u32>;
pub type Vec3f = Vec3<N32>;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<N>([N; 2]);

impl<N> Vec2<N> {
    #[inline]
    pub fn new(x: N, y: N) -> Self {
        Vec2([x, y])
    }

    #[inline]
    pub fn set<V: Into<Self>>(&mut self, v: V) {
        *self = v.into();
    }
}

impl<N: Num> Vec2<N> {
    #[inline]
    pub fn x(&self) -> N { self.0[0] }

    #[inline]
    pub fn y(&self) -> N { self.0[1] }

    #[inline]
    pub fn broadcast(v: N) -> Self {
        Vec2([v, v])
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> N {
        self.x() * rhs.x() + self.y() * rhs.y()
    }

    #[inline]
    pub fn len_sq(&self) -> N {
        self.dot(self)
    }

    #[inline]
    pub fn permute(&self, x: usize, y: usize) -> Self {
        Vec2([self[x], self[y]])
    }

    #[inline]
    pub fn max(&self, rhs: &Self) -> Self {
        Vec2([
            cmp::max(self.x(), rhs.x()),
            cmp::max(self.y(), rhs.y()),
        ])
    }

    #[inline]
    pub fn min(&self, rhs: &Self) -> Self {
        Vec2([
            cmp::min(self.x(), rhs.x()),
            cmp::min(self.y(), rhs.y()),
        ])
    }

    #[inline]
    pub fn max_val(&self) -> N {
        cmp::max(self.x(), self.y())
    }

    #[inline]
    pub fn min_val(&self) -> N {
        cmp::min(self.x(), self.y())
    }

    #[inline]
    pub fn max_dim(&self) -> usize {
        if self.x() >= self.y() { 0 } else { 1 }
    }

    #[inline]
    pub fn min_dim(&self) -> usize {
        if self.x() <= self.y() { 0 } else { 1 }
    }
}

impl <N: Num + Signed> Vec2<N> {
    #[inline]
    pub fn abs(&self) -> Self {
        Vec2([self.x().abs(), self.y().abs()])
    }
}

impl<N: Num + Float> Vec2<N> {
    #[inline]
    pub fn distance(&self, rhs: &Self) -> N {
        let dx = rhs.x() - self.x();
        let dy = rhs.y() - self.y();
        (dx * dx + dy * dy).sqrt()
    }

    #[inline]
    pub fn len(&self) -> N {
        self.len_sq().sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        self / self.len()
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        Vec2([
            self.x().ceil(),
            self.y().ceil(),
        ])
    }

    #[inline]
    pub fn floor(&self) -> Self {
        Vec2([
            self.x().floor(),
            self.y().floor(),
        ])
    }
}

impl<N: Num + Neg<Output = N>> Neg for Vec2<N> {
    type Output = Vec2<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec2([-self.x(), -self.y()])
    }
}

impl<N: Num + Neg<Output = N>> Neg for &Vec2<N> {
    type Output = Vec2<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec2([-self.x(), -self.y()])
    }
}

impl<N> Index<usize> for Vec2<N> {
    type Output = N;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<N> IndexMut<usize> for Vec2<N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<N, M> From<[M; 2]> for Vec2<N>
    where M: Clone + Into<N>
{
    #[inline]
    fn from(v: [M; 2]) -> Vec2<N> {
        Vec2::new(
            v[0].clone().into(),
            v[1].clone().into(),
        )
    }
}

impl<N, X, Y> From<(X, Y)> for Vec2<N>
    where X: Into<N>,
          Y: Into<N>,
{
    #[inline]
    fn from((x, y): (X, Y)) -> Vec2<N> {
        Vec2::new(x.into(), y.into())
    }
}

impl_all!(impl_add_v2v, Vec2<N>, Vec2<N>);
impl_mut!(impl_add_assign_v2v, Vec2<N>, Vec2<N>);

impl_all!(impl_sub_v2v, Vec2<N>, Vec2<N>);
impl_mut!(impl_sub_assign_v2v, Vec2<N>, Vec2<N>);

impl_all!(impl_mul_v2s, Vec2<N>, N);
impl_mut!(impl_mul_assign_v2s, Vec2<N>, N);

impl_all!(impl_div_v2s, Vec2<N>, N);
impl_mut!(impl_div_assign_v2s, Vec2<N>, N);

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3<N>([N; 3]);

impl<N> Vec3<N> {
    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Vec3([x, y, z])
    }

    #[inline]
    pub fn set<V: Into<Self>>(&mut self, v: V) {
        *self = v.into();
    }
}

impl<N: Num> Vec3<N> {
    #[inline]
    pub fn x(&self) -> N { self.0[0] }

    #[inline]
    pub fn y(&self) -> N { self.0[1] }

    #[inline]
    pub fn z(&self) -> N { self.0[2] }

    #[inline]
    pub fn broadcast(v: N) -> Self {
        Vec3([v, v, v])
    }

    #[inline]
    pub fn map<F, M>(&self, f: F) -> Vec3<M> where F: Fn(N) -> M {
        Vec3([
            f(self.x()),
            f(self.y()),
            f(self.z()),
        ])
    }

    #[inline]
    pub fn dot_v(&self, v: &Self) -> N {
        self.x() * v.x() +
        self.y() * v.y() +
        self.z() * v.z()
    }

    #[inline]
    pub fn dot_n(&self, n: &Normal3<N>) -> N {
        self.dot_v(&Vec3::from(*n))
    }

    #[inline]
    pub fn len_sq(&self) -> N {
        self.dot_v(self)
    }

    #[inline]
    pub fn permute(&self, x: usize, y: usize, z: usize) -> Self {
        Vec3([self[x], self[y], self[z]])
    }

    #[inline]
    pub fn max(&self, rhs: &Self) -> Self {
        Vec3([
            cmp::max(self.x(), rhs.x()),
            cmp::max(self.y(), rhs.y()),
            cmp::max(self.z(), rhs.z()),
        ])
    }

    #[inline]
    pub fn min(&self, rhs: &Self) -> Self {
        Vec3([
            cmp::min(self.x(), rhs.x()),
            cmp::min(self.y(), rhs.y()),
            cmp::min(self.z(), rhs.z()),
        ])
    }

    #[inline]
    pub fn max_val(&self) -> N {
        cmp::max(self.x(), cmp::max(self.y(), self.z()))
    }

    #[inline]
    pub fn min_val(&self) -> N {
        cmp::min(self.x(), cmp::min(self.y(), self.z()))
    }

    #[inline]
    pub fn max_dim(&self) -> usize {
        if self.x() >= self.y() {
            if self.x() >= self.z() { 0 } else { 2 }
        } else {
            if self.y() >= self.z() { 1 } else { 2 } 
        }
    }

    #[inline]
    pub fn min_dim(&self) -> usize {
        if self.x() <= self.y() {
            if self.x() <= self.z() { 0 } else { 2 }
        } else {
            if self.y() <= self.z() { 1 } else { 2 } 
        }
    }
}


impl <N: Num + Signed> Vec3<N> {
    #[inline]
    pub fn abs(&self) -> Self {
        Vec3([self.x().abs(), self.y().abs(), self.z().abs()])
    }
}

impl<N: Num + Float> Vec3<N> {
    #[inline]
    pub fn distance(&self, rhs: &Self) -> N {
        let dx = rhs.x() - self.x();
        let dy = rhs.y() - self.y();
        let dz = rhs.z() - self.z();
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    #[inline]
    pub fn len(&self) -> N {
        self.len_sq().sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        self / self.len()
    }

    #[inline]
    pub fn face_v(&self, v: &Vec3<N>) -> Self {
        if self.dot_v(v) < N::zero() { -self } else { *self }
    }

    #[inline]
    pub fn face_n(&self, n: &Normal3<N>) -> Self {
        if self.dot_n(n) < N::zero() { -self } else { *self }
    }

    #[inline]
    pub fn cross_v(&self, v: &Vec3<N>) -> Self {
        Vec3([
            self.y() * v.z() - self.z() * v.y(), 
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        ])
    }

    #[inline]
    pub fn cross_n(&self, n: &Normal3<N>) -> Self {
        self.cross_v(&Vec3::from(*n))
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        Vec3([
            self.x().ceil(),
            self.y().ceil(),
            self.z().ceil(),
        ])
    }

    #[inline]
    pub fn floor(&self) -> Self {
        Vec3([
            self.x().floor(),
            self.y().floor(),
            self.z().floor(),
        ])
    }
}

impl<N: Num + Neg<Output = N>> Neg for Vec3<N> {
    type Output = Vec3<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3([-self.x(), -self.y(), -self.z()])
    }
}

impl<N: Num + Neg<Output = N>> Neg for &Vec3<N> {
    type Output = Vec3<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3([-self.x(), -self.y(), -self.z()])
    }
}

impl<N> Index<usize> for Vec3<N> {
    type Output = N;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<N> IndexMut<usize> for Vec3<N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<N, M> From<[M; 3]> for Vec3<N>
    where M: Clone + Into<N>
{
    #[inline]
    fn from(v: [M; 3]) -> Vec3<N> {
        Vec3::new(
            v[0].clone().into(),
            v[1].clone().into(),
            v[2].clone().into()
        )
    }
}

impl<N, X, Y, Z> From<(X, Y, Z)> for Vec3<N>
    where X: Into<N>,
          Y: Into<N>,
          Z: Into<N>,
{
    #[inline]
    fn from((x, y, z): (X, Y, Z)) -> Vec3<N> {
        Vec3::new(x.into(), y.into(), z.into())
    }
}

impl_all!(impl_add_v3v, Vec3<N>, Vec3<N>);
impl_mut!(impl_add_assign_v3v, Vec3<N>, Vec3<N>);

impl_all!(impl_sub_v3v, Vec3<N>, Vec3<N>);
impl_mut!(impl_sub_assign_v3v, Vec3<N>, Vec3<N>);

impl_all!(impl_mul_v3s, Vec3<N>, N);
impl_mut!(impl_mul_assign_v3s, Vec3<N>, N);

impl_all!(impl_div_v3s, Vec3<N>, N);
impl_mut!(impl_div_assign_v3s, Vec3<N>, N);
