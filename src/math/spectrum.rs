use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Index, IndexMut,
    Neg,
};

use num_traits::Float;
use noisy_float::prelude::*;

use crate::math::{clamp, Num, Vec3};

pub type Spectrum3f = Spectrum3<N32>;

#[derive(Copy, Clone, Debug)]
pub struct Spectrum3<N>(Vec3<N>);

impl<N: Num> Spectrum3<N> {

    #[inline]
    fn x(&self) -> N { self.0.x() } 

    #[inline]
    fn y(&self) -> N { self.0.y() } 

    #[inline]
    fn z(&self) -> N { self.0.z() } 

    #[inline]
    pub fn r(&self) -> N { self.0.x() }

    #[inline]
    pub fn g(&self) -> N { self.0.y() }

    #[inline]
    pub fn b(&self) -> N { self.0.z() }

    #[inline]
    pub fn set<V: Into<Self>>(&mut self, v: V) {
        *self = v.into();
    }

    pub fn is_black(&self) -> bool {
        for i in 0..3 {
            if self[i] != N::zero() { return false }
        }
        true
    }
}

impl<N: Num + Float> Spectrum3<N> {
    pub fn sqrt(&self) -> Self {
        Spectrum3(self.0.map(|n| n.sqrt()))
    }

    pub fn lerp(&self, spectrum: Self, t: N) -> Self {
        self * (N::one() - t) + spectrum * t
    }

    pub fn clamp(&self, lo: N, hi: N) -> Self {
        Spectrum3(self.0.map(|n| clamp(n, lo, hi)))
    }
}

impl<N> Index<usize> for Spectrum3<N> {
    type Output = N; 
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<N> IndexMut<usize> for Spectrum3<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<N: Num + Neg<Output = N>> Neg for Spectrum3<N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Spectrum3(-self.0)
    }
}

impl<N, M> From<[M; 3]> for Spectrum3<N>
    where M: Clone + Into<N>
{
    #[inline]
    fn from(v: [M; 3]) -> Spectrum3<N> {
        Spectrum3(Vec3::from(v))
    }
}

impl<N, X, Y, Z> From<(X, Y, Z)> for Spectrum3<N>
    where X: Into<N>,
          Y: Into<N>,
          Z: Into<N>,
{
    #[inline]
    fn from(p: (X, Y, Z)) -> Spectrum3<N> {
        Spectrum3(Vec3::from(p))        
    }
}

impl_all!(impl_add_v3v, Spectrum3<N>, Spectrum3<N>);
impl_mut!(impl_add_assign_v3v, Spectrum3<N>, Spectrum3<N>);
impl_all!(impl_sub_v3v, Spectrum3<N>, Spectrum3<N>);
impl_mut!(impl_sub_assign_v3v, Spectrum3<N>, Spectrum3<N>);
impl_all!(impl_mul_v3v, Spectrum3<N>, Spectrum3<N>);
impl_mut!(impl_mul_assign_v3v, Spectrum3<N>, Spectrum3<N>);
impl_all!(impl_div_v3v, Spectrum3<N>, Spectrum3<N>);
impl_mut!(impl_div_assign_v3v, Spectrum3<N>, Spectrum3<N>);

impl_all!(impl_mul_v3s, Spectrum3<N>, N);
impl_mut!(impl_mul_assign_v3s, Spectrum3<N>, N);
impl_all!(impl_div_v3s, Spectrum3<N>, N);
impl_mut!(impl_div_assign_v3s, Spectrum3<N>, N);
