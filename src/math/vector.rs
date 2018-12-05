use std::fmt;

pub trait Num:
    Copy +
    Clone +
    Eq +
    PartialOrd +
    Ord +
    fmt::Debug +
    fmt::Display +
    num_traits::Num +
    num_traits::NumRef +
    num_traits::NumAssignRef {}

pub struct Vector2<N: Num> {
    x: N,
    y: N,
}

pub struct Vector3<N: Num> {
    x: N,
    y: N,
    z: N,
}

pub type Vector2i = Vector2<u32>;
pub type Vector2f = Vector2<f32>;
pub type Vector3i = Vector3<u32>;
pub type Vector3f = Vector3<f32>;
