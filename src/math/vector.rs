#![allow(dead_code)]

use noisy_float::prelude::*;

pub trait Num:
    Copy
    + Clone
    + Default
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + std::hash::Hash
    + std::fmt::Debug
    + std::fmt::Display
    + num_traits::Num
    + num_traits::NumOps
    + num_traits::NumAssign
{
}

impl<T> Num for T where
    T: Copy
        + Clone
        + Default
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + std::hash::Hash
        + std::fmt::Debug
        + std::fmt::Display
        + num_traits::Num
        + num_traits::NumOps
        + num_traits::NumAssign
{
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector2<N: Num> {
    pub x: N,
    pub y: N,
}

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
