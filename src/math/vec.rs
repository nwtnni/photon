use std::arch::x86_64;
use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    Neg,
    Not,
    Deref,
};

use crate::math;

#[derive(Copy, Clone, Debug)]
pub struct Vec3(x86_64::__m128);

impl Default for Vec3 {
    fn default() -> Self {
        unsafe {
            Vec3(x86_64::_mm_set1_ps(0.0))
        }
    }
}

impl Vec3 {
    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        unsafe {
            Vec3(x86_64::_mm_set_ps(0.0, z, y, x))
        }
    }

    #[inline(always)]
    pub fn inv(&self) -> Self {
        unsafe {
            let num = x86_64::_mm_set1_ps(1.0);
            let den = self.0;
            Vec3(x86_64::_mm_div_ps(num, den))
        }
    }

    #[inline(always)]
    pub fn broadcast(x: f32) -> Self {
        unsafe {
            Vec3(x86_64::_mm_set1_ps(x))
        }
    }

    pub fn get(&self, index: usize) -> f32 {
        match index {
        | 0 => self.x(),
        | 1 => self.y(),
        | 2 => self.z(),
        | _ => unreachable!("[INTERNAL ERROR]: Vec3 index out of bounds"),
        }
    }

    #[inline(always)]
    pub fn x(&self) -> f32 {
        unsafe {
            x86_64::_mm_cvtss_f32(self.0)
        }
    }

    #[inline(always)]
    pub fn y(&self) -> f32 {
        unsafe {
            x86_64::_mm_cvtss_f32(
                x86_64::_mm_permute_ps(self.0, 0b01_01_01_01)
            )
        }
    }

    #[inline(always)]
    pub fn z(&self) -> f32 {
        unsafe {
            x86_64::_mm_cvtss_f32(
                x86_64::_mm_permute_ps(self.0, 0b10_10_10_10)
            )
        }
    }

    #[inline(always)]
    pub fn r(&self) -> f32 { self.x() }

    #[inline(always)]
    pub fn g(&self) -> f32 { self.y() }

    #[inline(always)]
    pub fn b(&self) -> f32 { self.z() }

    #[inline(always)]
    pub fn sqrt(&self) -> Self {
        unsafe {
            Vec3(x86_64::_mm_sqrt_ps(self.0))
        }
    }

    #[inline(always)]
    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    #[inline(always)]
    pub fn len_sq(&self) -> f32 { self.dot(self) }

    pub fn dot(&self, rhs: &Self) -> f32 {
        unsafe {
            let dot = x86_64::_mm_dp_ps(self.0, rhs.0, 0b0111_0001);
            x86_64::_mm_cvtss_f32(dot)
        }
    }

    // See: https://geometrian.com/programming/tutorials/cross-product/index.php
    pub fn cross(&self, rhs: &Self) -> Self {
        unsafe {
            // 3 0 2 1
            const PERMUTE: i32 = 0b11_00_10_01;

            // y2
            // z2
            // x2
            // 0.0
            let mut l = x86_64::_mm_permute_ps(rhs.0, PERMUTE);

            // y1
            // z1
            // x1
            // 0.0
            let mut r = x86_64::_mm_permute_ps(self.0, PERMUTE);

            // x1 * y2
            // y1 * z2
            // z1 * x2
            // 0.0 * 0.0
            l = x86_64::_mm_mul_ps(self.0, l);

            // y1 * x2
            // z1 * y2
            // x1 * z2
            // 0.0 * 0.0
            r = x86_64::_mm_mul_ps(r, rhs.0);

            // x1 * y2 - y1 * x2
            // y1 * z2 - z1 * y2
            // z1 * x2 - x1 * z2
            // 0.0 * 0.0 - 0.0 * 0.0
            let sub = x86_64::_mm_sub_ps(l, r);

            // y1 * z2 - z1 * y2
            // z1 * x2 - x1 * z2
            // x1 * y2 - y1 * x2
            // 0.0 * 0.0 - 0.0 * 0.0
            Vec3(x86_64::_mm_permute_ps(sub, PERMUTE))
        }
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        unsafe {
            let len = x86_64::_mm_set1_ps(self.len());
            Vec3(x86_64::_mm_div_ps(self.0, len))
        }
    }

    #[inline(always)]
    pub fn lerp(&self, to: &Self, t: f32) -> Self {
        self * (1.0 - t) + to * t
    }

    // See: https://stackoverflow.com/questions/32408665/fastest-way-to-compute-absolute-value-using-sse
    pub fn abs(&self) -> Self {
        unsafe {
            let abs = x86_64::_mm_castsi128_ps(x86_64::_mm_set1_epi32(!(1 << 31)));
            Vec3(x86_64::_mm_and_ps(self.0, abs))
        }
    }

    pub fn min(&self, rhs: &Self) -> Self {
        unsafe {
            Vec3(x86_64::_mm_min_ps(self.0, rhs.0))
        }
    }

    pub fn max(&self, rhs: &Self) -> Self {
        unsafe {
            Vec3(x86_64::_mm_max_ps(self.0, rhs.0))
        }
    }

    pub fn min_horizontal(&self) -> f32 {
        unsafe {
            // min(x, y)
            // min(y, x)
            // min(z, z)
            // min(z, z)
            let half = x86_64::_mm_min_ps(
                self.0,
                x86_64::_mm_permute_ps(self.0, 0b10_10_00_01),
            );

            // min(min(x, y), min(z, z))
            // min(min(x, y), min(z, z))
            // min(min(z, z), min(x, y))
            // min(min(z, z), min(x, y))
            let full = x86_64::_mm_min_ps(
                half,
                x86_64::_mm_permute_ps(half, 0b00_00_10_10),
            );

            x86_64::_mm_cvtss_f32(full)
        }
    }

    pub fn max_horizontal(&self) -> f32 {
        unsafe {
            // max(x, y)
            // max(y, x)
            // max(z, z)
            // max(z, z)
            let half = x86_64::_mm_max_ps(
                self.0,
                x86_64::_mm_permute_ps(self.0, 0b10_10_00_01),
            );

            // max(max(x, y), max(z, z))
            // max(max(x, y), max(z, z))
            // max(max(z, z), max(x, y))
            // max(max(z, z), max(x, y))
            let full = x86_64::_mm_max_ps(
                half,
                x86_64::_mm_permute_ps(half, 0b00_00_10_10),
            );

            x86_64::_mm_cvtss_f32(full)
        }
    }

    pub fn is_zero(&self) -> bool {
        unsafe {
            let Vec3(abs) = self.abs();
            let eps = x86_64::_mm_set1_ps(math::EPSILON);
            match x86_64::_mm_test_all_ones(x86_64::_mm_cvttps_epi32(x86_64::_mm_cmplt_ps(abs, eps))) {
            | 0 => false,
            | _ => true,
            }
        }
    }

    pub fn gt(&self, rhs: &Self) -> Self {
        unsafe {
            Vec3(x86_64::_mm_cmpgt_ps(self.0, rhs.0))
        }
    }

    pub fn blend(&self, lhs: &Self, rhs: &Self) -> Self {
        unsafe {
            Vec3(x86_64::_mm_blendv_ps(lhs.0, rhs.0, self.0))
        }
    }
}

macro_rules! impl_op {
    ($op:ident, $fn:ident, $op_mut:ident, $fn_mut:ident, $ty:ty, $lhs:ty, $rhs:ty, $input:pat => $output:expr) => {
        impl $op<$rhs> for $lhs {
            type Output = Vec3;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                unsafe {
                    Vec3(match (self.0, rhs) { $input => $output })
                }
            }
        }

        impl $op_mut<$rhs> for $ty {
            fn $fn_mut(&mut self, rhs: $rhs) {
                unsafe {
                    self.0 = match (self.0, rhs) { $input => $output };
                }
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

impl_all!(Add, add, AddAssign, add_assign, Vec3, Vec3, (lhs, Vec3(ref rhs)) => x86_64::_mm_add_ps(lhs, *rhs));
impl_all!(Sub, sub, SubAssign, sub_assign, Vec3, Vec3, (lhs, Vec3(ref rhs)) => x86_64::_mm_sub_ps(lhs, *rhs));
impl_all!(Mul, mul, MulAssign, mul_assign, Vec3, Vec3, (lhs, Vec3(ref rhs)) => x86_64::_mm_mul_ps(lhs, *rhs));
impl_all!(Div, div, DivAssign, div_assign, Vec3, Vec3, (lhs, Vec3(ref rhs)) => x86_64::_mm_div_ps(lhs, *rhs));

impl_all!(BitAnd, bitand, BitAndAssign, bitand_assign, Vec3, Vec3, (lhs, Vec3(ref rhs)) => x86_64::_mm_and_ps(lhs, *rhs));
impl_all!(BitOr, bitor, BitOrAssign, bitor_assign, Vec3, Vec3, (lhs, Vec3(ref rhs)) => x86_64::_mm_or_ps(lhs, *rhs));

impl_all!(Mul, mul, MulAssign, mul_assign, Vec3, f32, (lhs, ref rhs) => {
    let rhs = x86_64::_mm_set1_ps(*rhs.deref());
    x86_64::_mm_mul_ps(lhs, rhs)
});

impl_all!(Div, div, DivAssign, div_assign, Vec3, f32, (lhs, ref rhs) => {
    let rhs = x86_64::_mm_set1_ps(*rhs.deref());
    x86_64::_mm_div_ps(lhs, rhs)
});

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        unsafe { 
            let sign = x86_64::_mm_castsi128_ps(x86_64::_mm_set1_epi32(1 << 31));
            Vec3(x86_64::_mm_xor_ps(self.0, sign))
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        unsafe {
            let sign = x86_64::_mm_castsi128_ps(x86_64::_mm_set1_epi32(1 << 31));
            Vec3(x86_64::_mm_xor_ps(self.0, sign))
        }
    }
}

impl Not for Vec3 {
    type Output = Vec3;
    fn not(self) -> Self::Output {
        unsafe {
            let xor = x86_64::_mm_castsi128_ps(x86_64::_mm_set1_epi32(-1));
            Vec3(x86_64::_mm_xor_ps(self.0, xor))
        }
    }
}

impl Not for &Vec3 {
    type Output = Vec3;
    fn not(self) -> Self::Output {
        unsafe {
            let xor = x86_64::_mm_castsi128_ps(x86_64::_mm_set1_epi32(-1));
            Vec3(x86_64::_mm_xor_ps(self.0, xor))
        }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    #[inline(always)]
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Vec3::new(x, y, z)
    }
}

impl From<[f32; 3]> for Vec3 {
    #[inline(always)]
    fn from([x, y, z]: [f32; 3]) -> Vec3 {
        Vec3::new(x, y, z)
    }
}
