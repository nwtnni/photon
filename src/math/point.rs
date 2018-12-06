use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Index, IndexMut,
    Neg,
};

use noisy_float::prelude::*;
use serde_derive::{Serialize, Deserialize};

use crate::math::{Num, Vector2, Vector3};

pub type Point2i = Vector2<u32>;
pub type Point2f = Vector2<N32>;
pub type Point3i = Vector3<u32>;
pub type Point3f = Vector3<N32>;

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2<N: Num>(Vector2<N>);

impl<N: Num> Point2<N> {
    #[inline]
    pub fn new(x: N, y: N) -> Self {
        Point2(Vector2::new(x, y)) 
    }

    #[inline]
    pub fn x(&self) -> N { self.0.x() }

    #[inline]
    pub fn y(&self) -> N { self.0.y() }
}

impl<N: Num> From<Point3<N>> for Point2<N> {
    fn from(p: Point3<N>) -> Point2<N> {
        Point2(Vector2::new(p.x(), p.y()))
    }
}

impl_all_pairs!(impl_p2_add, Point2<N>, Vector2<N>);
impl_pairs!(impl_p2_add_assign, Point2<N>, Vector2<N>);
impl_all_pairs!(impl_p2_sub, Point2<N>, Point2<N>);

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3<N: Num>(Vector3<N>);

impl<N: Num> Point3<N> {
    #[inline]
    pub fn new(x: N, y: N, z: N) -> Self {
        Point3(Vector3::new(x, y, z)) 
    }

    #[inline]
    pub fn x(&self) -> N { self.0.x() }

    #[inline]
    pub fn y(&self) -> N { self.0.y() }

    #[inline]
    pub fn z(&self) -> N { self.0.z() }
}
