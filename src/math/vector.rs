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
    real::Real,        
    sign::Signed,
};

use crate::math::{Num, Normal3};

pub type Vec2i = Vec2<u32>;
pub type Vec2f = Vec2<N32>;
pub type Vec2d = Vec2<N64>;
pub type Vec3i = Vec3<u32>;
pub type Vec3f = Vec3<N32>;
pub type Vec3d = Vec3<N64>;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<N> {
    x: N,
    y: N,
}

impl<N: Num> Vec2<N> {

    #[inline]
    pub fn unpack((x, y): (N, N)) -> Self {
        Self::new(x, y) 
    }

    #[inline]
    pub fn pack(&mut self, (x, y): (N, N)) {
        self.set(x, y)
    }

    #[inline]
    pub fn x(&self) -> N { self.x }

    #[inline]
    pub fn y(&self) -> N { self.y }

    #[inline]
    pub fn set(&mut self, x: N, y: N) {
        self.x = x;
        self.y = y;
    }

    #[inline]
    pub fn new(x: N, y: N) -> Self {
        Vec2 { x, y }
    }

    #[inline]
    pub fn fill(v: N) -> Self {
        Vec2 { x: v, y: v }
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> N {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    pub fn len_sq(&self) -> N {
        self.dot(self)
    }

    #[inline]
    pub fn permute(&self, x: usize, y: usize) -> Self {
        Vec2 {
            x: self[x],
            y: self[y],
        }
    }

    #[inline]
    pub fn max(&self, rhs: &Self) -> Self {
        Vec2 {
            x: cmp::max(self.x, rhs.x),
            y: cmp::max(self.y, rhs.y),
        }
    }

    #[inline]
    pub fn min(&self, rhs: &Self) -> Self {
        Vec2 {
            x: cmp::min(self.x, rhs.x),
            y: cmp::min(self.y, rhs.y),
        }
    }

    #[inline]
    pub fn max_val(&self) -> N {
        cmp::max(self.x, self.y)
    }

    #[inline]
    pub fn min_val(&self) -> N {
        cmp::min(self.x, self.y)
    }

    #[inline]
    pub fn max_dim(&self) -> usize {
        if self.x >= self.y { 0 } else { 1 }
    }

    #[inline]
    pub fn min_dim(&self) -> usize {
        if self.x <= self.y { 0 } else { 1 }
    }
}

impl <N: Signed> Vec2<N> {
    #[inline]
    pub fn abs(&self) -> Self {
        Vec2 { x: self.x.abs(), y: self.y.abs() }
    }
}

impl<N: Num + Real> Vec2<N> {
    #[inline]
    pub fn distance(&self, rhs: &Self) -> N {
        let dx = rhs.x - self.x;
        let dy = rhs.y - self.y;
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
        Vec2 {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    #[inline]
    pub fn floor(&self) -> Self {
        Vec2 {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
}

impl<N: Num + Neg<Output = N>> Neg for Vec2<N> {
    type Output = Vec2<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec2 { x: -self.x(), y: -self.y() }
    }
}

impl<N: Num + Neg<Output = N>> Neg for &Vec2<N> {
    type Output = Vec2<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec2 { x: -self.x(), y: -self.y() }
    }
}

impl<N> Index<usize> for Vec2<N> {
    type Output = N;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
        | 0 => &self.x,
        | 1 => &self.y,
        | n => panic!("[INTERNAL ERROR]: invalid index {}", n),
        }
    }
}

impl<N> IndexMut<usize> for Vec2<N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
        | 0 => &mut self.x,
        | 1 => &mut self.y,
        | n => panic!("[INTERNAL ERROR]: invalid index {}", n),
        }
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
pub struct Vec3<N> {
    x: N,
    y: N,
    z: N,
}

impl<N: Num> Vec3<N> {

    #[inline]
    pub fn unpack((x, y, z): (N, N, N)) -> Self {
        Self::new(x, y, z)
    }

    #[inline]
    pub fn pack(&mut self, (x, y, z): (N, N, N)) {
        self.set(x, y, z)
    }

    #[inline]
    pub fn x(&self) -> N { self.x }

    #[inline]
    pub fn y(&self) -> N { self.y }

    #[inline]
    pub fn z(&self) -> N { self.z }

    #[inline]
    pub fn set(&mut self, x: N, y: N, z: N) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn fill(v: N) -> Self {
        Vec3 { x: v, y: v, z: v }
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
        Vec3 {
            x: self[x],
            y: self[y],
            z: self[z],
        }
    }

    #[inline]
    pub fn max(&self, rhs: &Self) -> Self {
        Vec3 {
            x: cmp::max(self.x, rhs.x),
            y: cmp::max(self.y, rhs.y),
            z: cmp::max(self.z, rhs.z),
        }
    }

    #[inline]
    pub fn min(&self, rhs: &Self) -> Self {
        Vec3 {
            x: cmp::min(self.x, rhs.x),
            y: cmp::min(self.y, rhs.y),
            z: cmp::min(self.z, rhs.z),
        }
    }

    #[inline]
    pub fn max_val(&self) -> N {
        cmp::max(self.x, cmp::max(self.y, self.z))
    }

    #[inline]
    pub fn min_val(&self) -> N {
        cmp::min(self.x, cmp::min(self.y, self.z))
    }

    #[inline]
    pub fn max_dim(&self) -> usize {
        if self.x >= self.y {
            if self.x >= self.z { 0 } else { 2 }
        } else {
            if self.y >= self.z { 1 } else { 2 } 
        }
    }

    #[inline]
    pub fn min_dim(&self) -> usize {
        if self.x <= self.y {
            if self.x <= self.z { 0 } else { 2 }
        } else {
            if self.y <= self.z { 1 } else { 2 } 
        }
    }
}


impl <N: Signed> Vec3<N> {
    #[inline]
    pub fn abs(&self) -> Self {
        Vec3 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }
}

impl<N: Num + Real> Vec3<N> {
    #[inline]
    pub fn distance(&self, rhs: &Self) -> N {
        let dx = rhs.x - self.x;
        let dy = rhs.y - self.y;
        let dz = rhs.z - self.z;
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
        Vec3 {
            x: self.y() * v.z() - self.z() * v.y(), 
            y: self.z() * v.x() - self.x() * v.z(),
            z: self.x() * v.y() - self.y() * v.x(),
        }
    }

    #[inline]
    pub fn cross_n(&self, n: &Normal3<N>) -> Self {
        self.cross_v(&Vec3::from(*n))
    }

    #[inline]
    pub fn ceil(&self) -> Self {
        Vec3 {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    #[inline]
    pub fn floor(&self) -> Self {
        Vec3 {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }
}

impl<N: Num + Neg<Output = N>> Neg for Vec3<N> {
    type Output = Vec3<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl<N: Num + Neg<Output = N>> Neg for &Vec3<N> {
    type Output = Vec3<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x(), y: -self.y(), z: -self.z() }
    }
}

impl<N> Index<usize> for Vec3<N> {
    type Output = N;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        match i {
        | 0 => &self.x,
        | 1 => &self.y,
        | 2 => &self.z,
        | n => panic!("[INTERNAL ERROR]: invalid index {}", n),
        }
    }
}

impl<N> IndexMut<usize> for Vec3<N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
        | 0 => &mut self.x,
        | 1 => &mut self.y,
        | 2 => &mut self.z,
        | n => panic!("[INTERNAL ERROR]: invalid index {}", n),
        }
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
