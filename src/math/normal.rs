use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Index, IndexMut,
    Neg,
};

use serde_derive::{Serialize, Deserialize};
use num_traits::{
    real::Real,
    sign::Signed,
};

use crate::math::{Num, Vector3};

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Normal3<N>(Vector3<N>);

impl<N: Num> Normal3<N> {
    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Normal3(Vector3::new(x, y, z))
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
    pub fn len_sq(&self) -> N {
        self.0.len_sq()
    }
}

impl<N: Num + Real> Normal3<N> {
    #[inline]
    pub fn len(&self) -> N {
        self.0.len()
    }
}

impl<N> From<Vector3<N>> for Normal3<N> {
    #[inline]
    fn from(v: Vector3<N>) -> Self {
        Normal3(v)
    }
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

impl_nn3!(Add, AddAssign, add, add_assign, ((x1, y1, z1), (x2, y2, z2)) => (x1 + x2, y1 + y2, z1 + z2));
impl_nn3!(Sub, SubAssign, sub, sub_assign, ((x1, y1, z1), (x2, y2, z2)) => (x1 - x2, y1 - y2, z1 + z2));
impl_ns3!(Mul, MulAssign, mul, mul_assign, ((x, y, z), s) => (x * s, y * s, z * s));
impl_ns3!(Div, DivAssign, div, div_assign, ((x, y, z), s) => (x / s, y / s, z / s));
