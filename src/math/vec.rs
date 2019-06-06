use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Index, IndexMut,
    Neg
};

use crate::math::{min, max};

pub const ONES_3D: Vec3 = Vec3([1.0, 1.0, 1.0]);
pub const ONES_2D: Vec3 = Vec3([1.0, 1.0, 0.0]);

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec3([f32; 3]);

impl Vec3 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3([x, y, z])
    }

    #[inline(always)]
    pub fn broadcast(x: f32) -> Self {
        Vec3([x, x, x])
    }

    #[inline(always)]
    pub fn set<V: Into<Vec3>>(&mut self, v: V) {
        *self = v.into()
    }

    #[inline(always)]
    pub fn x(&self) -> f32 { self.0[0] }

    #[inline(always)]
    pub fn y(&self) -> f32 { self.0[1] }

    #[inline(always)]
    pub fn z(&self) -> f32 { self.0[2] }

    #[inline(always)]
    pub fn r(&self) -> f32 { self.0[0] }

    #[inline(always)]
    pub fn g(&self) -> f32 { self.0[1] }

    #[inline(always)]
    pub fn b(&self) -> f32 { self.0[2] }

    #[inline(always)]
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    #[inline(always)]
    pub fn len_sq(&self) -> f32 { self.dot(self) }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() +
        self.y() * rhs.y() +
        self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3([
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ])
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        self / self.len()
    }

    #[inline(always)]
    pub fn lerp(&self, to: &Self, t: f32) -> Self {
        self * (1.0 - t) + to * t
    }

    pub fn min(&self, rhs: &Self) -> Self {
        Vec3([
            min(self[0], rhs[0]),
            min(self[1], rhs[1]),
            min(self[2], rhs[2]),
        ])
    }

    pub fn max(&self, rhs: &Self) -> Self {
        Vec3([
            max(self[0], rhs[0]),
            max(self[1], rhs[1]),
            max(self[2], rhs[2]),
        ])
    }
}

macro_rules! impl_op {
    ($op:ident, $fn:ident, $op_mut:ident, $fn_mut:ident, $ty:ty, $lhs:ty, $rhs:ty, $input:pat => $output:expr) => {
        impl $op<$rhs> for $lhs {
            type Output = Vec3;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                Vec3::from(match (self.0, rhs) { $input => $output })
            }
        }

        impl $op_mut<$rhs> for $ty {
            fn $fn_mut(&mut self, rhs: $rhs) {
                self.set(Vec3::from(match (self.0, rhs) { $input => $output }))
            }
        }
    }
}

macro_rules! impl_all {
    ($op:ident, $fn:ident, $op_mut:ident, $fn_mut:ident, $lhs:ty, $rhs:ty, $input:pat => $output:expr) => {
        impl_op!($op, $fn, $op_mut, $fn_mut, $lhs, $lhs, $rhs, $input => $output);
        impl_op!($op, $fn, $op_mut, $fn_mut, $lhs, $lhs, &$rhs, $input => $output);
        impl_op!($op, $fn, $op_mut, $fn_mut, &mut $lhs, &$lhs, $rhs, $input => $output);
        impl_op!($op, $fn, $op_mut, $fn_mut, &mut $lhs, &$lhs, &$rhs, $input => $output);
    }
}

impl_all!(Add, add, AddAssign, add_assign, Vec3, Vec3, ([x1, y1, z1], Vec3([x2, y2, z2])) => [x1 + x2, y1 + y2, z1 + z2]);
impl_all!(Sub, sub, SubAssign, sub_assign, Vec3, Vec3, ([x1, y1, z1], Vec3([x2, y2, z2])) => [x1 - x2, y1 - y2, z1 - z2]);
impl_all!(Mul, mul, MulAssign, mul_assign, Vec3, Vec3, ([x1, y1, z1], Vec3([x2, y2, z2])) => [x1 * x2, y1 * y2, z1 * z2]);
impl_all!(Div, div, DivAssign, div_assign, Vec3, Vec3, ([x1, y1, z1], Vec3([x2, y2, z2])) => [x1 / x2, y1 / y2, z1 / z2]);
impl_all!(Mul, mul, MulAssign, mul_assign, Vec3, f32, ([x, y, z], t) => [x * t, y * t, z * t]);
impl_all!(Div, div, DivAssign, div_assign, Vec3, f32, ([x, y, z], t) => [x / t, y / t, z / t]);

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3([-self[0], -self[1], -self[2]])
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3([-self[0], -self[1], -self[2]])
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    #[inline(always)]
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Vec3 {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline(always)]
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Vec3([x, y, z])
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline(always)]
    fn from(xyz: [f32; 3]) -> Vec3 {
        Vec3(xyz)
    }
}
