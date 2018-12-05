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

use noisy_float::prelude::*;
use num_traits::{
    real::Real,        
    sign::Signed,
};

use crate::math::Num;

pub type Vector2i = Vector2<u32>;
pub type Vector2f = Vector2<N32>;
pub type Vector3i = Vector3<u32>;
pub type Vector3f = Vector3<N32>;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector2<N: Num> {
    pub x: N,
    pub y: N,
}

impl<N: Num> Vector2<N> {
    #[inline]
    pub fn fill(v: N) -> Self {
        Vector2 { x: v, y: v }
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
        Vector2 {
            x: self[x],
            y: self[y],
        }
    }
}

impl <N: Num + cmp::Ord> Vector2<N> {
    #[inline]
    pub fn max(&self, rhs: &Self) -> Self {
        Vector2 {
            x: cmp::max(self.x, rhs.x),
            y: cmp::max(self.y, rhs.y),
        }
    }

    #[inline]
    pub fn min(&self, rhs: &Self) -> Self {
        Vector2 {
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

impl <N: Num + Signed> Vector2<N> {
    pub fn abs(&self) -> Self {
        Vector2 { x: self.x.abs(), y: self.y.abs() }
    }
}

impl<N: Num + Real> Vector2<N> {
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
        *self / self.len()
    }
}

impl<N: Num + Neg<Output = N>> Neg for Vector2<N> {
    type Output = Vector2<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector2 { x: -self.x, y: -self.y }
    }
}

impl<N: Num> Index<usize> for Vector2<N> {
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

impl<N: Num> IndexMut<usize> for Vector2<N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
        | 0 => &mut self.x,
        | 1 => &mut self.y,
        | n => panic!("[INTERNAL ERROR]: invalid index {}", n),
        }
    }
}

impl_vv2!(Add, AddAssign, add, add_assign, ((x1, y1), (x2, y2)) => (x1 + x2, y1 + y2));
impl_vv2!(Sub, SubAssign, sub, sub_assign, ((x1, y1), (x2, y2)) => (x1 - x2, y1 - y2));
impl_vv2!(Mul, MulAssign, mul, mul_assign, ((x1, y1), (x2, y2)) => (x1 * x2, y1 * y2));
impl_vv2!(Div, DivAssign, div, div_assign, ((x1, y1), (x2, y2)) => (x1 / x2, y1 / y2));

impl_vs2!(Add, AddAssign, add, add_assign, ((x, y), s) => (x + s, y + s));
impl_vs2!(Sub, SubAssign, sub, sub_assign, ((x, y), s) => (x - s, y - s));
impl_vs2!(Mul, MulAssign, mul, mul_assign, ((x, y), s) => (x * s, y * s));
impl_vs2!(Div, DivAssign, div, div_assign, ((x, y), s) => (x / s, y / s));

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector3<N: Num> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N: Num> Vector3<N> {
    #[inline]
    fn fill(v: N) -> Self {
        Vector3 { x: v, y: v, z: v }
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> N {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn len_sq(&self) -> N {
        self.dot(self)
    }

    #[inline]
    pub fn permute(&self, x: usize, y: usize, z: usize) -> Self {
        Vector3 {
            x: self[x],
            y: self[y],
            z: self[z],
        }
    }
}

impl <N: Num + cmp::Ord> Vector3<N> {
    #[inline]
    pub fn max(&self, rhs: &Self) -> Self {
        Vector3 {
            x: cmp::max(self.x, rhs.x),
            y: cmp::max(self.y, rhs.y),
            z: cmp::max(self.z, rhs.z),
        }
    }

    #[inline]
    pub fn min(&self, rhs: &Self) -> Self {
        Vector3 {
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


impl <N: Num + Signed> Vector3<N> {
    #[inline]
    fn abs(&self) -> Self {
        Vector3 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }
}

impl<N: Num + Real> Vector3<N> {
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
        *self / self.len()
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y, 
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<N: Num + Neg<Output = N>> Neg for Vector3<N> {
    type Output = Vector3<N>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl<N: Num> Index<usize> for Vector3<N> {
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

impl<N: Num> IndexMut<usize> for Vector3<N> {
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

impl_vv3!(Add, AddAssign, add, add_assign, ((x1, y1, z1), (x2, y2, z2)) => (x1 + x2, y1 + y2, z1 + z2));
impl_vv3!(Sub, SubAssign, sub, sub_assign, ((x1, y1, z1), (x2, y2, z2)) => (x1 - x2, y1 - y2, z1 + z2));
impl_vv3!(Mul, MulAssign, mul, mul_assign, ((x1, y1, z1), (x2, y2, z2)) => (x1 * x2, y1 * y2, z1 * z2));
impl_vv3!(Div, DivAssign, div, div_assign, ((x1, y1, z1), (x2, y2, z2)) => (x1 / x2, y1 / y2, z1 / z2));

impl_vs3!(Add, AddAssign, add, add_assign, ((x, y, z), s) => (x + s, y + s, z + s));
impl_vs3!(Sub, SubAssign, sub, sub_assign, ((x, y, z), s) => (x - s, y - s, z - s));
impl_vs3!(Mul, MulAssign, mul, mul_assign, ((x, y, z), s) => (x * s, y * s, z * s));
impl_vs3!(Div, DivAssign, div, div_assign, ((x, y, z), s) => (x / s, y / s, z / s));
