#![allow(dead_code)]

use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub,
    SubAssign,
};

use noisy_float::prelude::*;

use crate::math::Num;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector2<N: Num> {
    pub x: N,
    pub y: N,
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

pub type Vector2i = Vector2<u32>;
pub type Vector2f = Vector2<N32>;
pub type Vector3i = Vector3<u32>;
pub type Vector3f = Vector3<N32>;
