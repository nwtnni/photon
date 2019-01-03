#[macro_use]
mod macros;
mod matrix;
mod normal;
mod point;
mod ray;
mod spectrum;
mod transform;
mod vector;

pub use self::normal::*;
pub use self::matrix::*;
pub use self::point::*;
pub use self::ray::*;
pub use self::spectrum::*;
pub use self::transform::*;
pub use self::vector::*;

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
    + num_traits::Bounded
{
    fn two() -> Self { Self::one() + Self::one() }
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
        + num_traits::Bounded
{
}

use noisy_float::prelude::*;

pub fn solve_quadratic(a: N32, b: N32, c: N32) -> Option<(N32, N32)> {
    let d_sq = b * b - n32(4.0) * a * c;

    if d_sq < n32(0.0) { return None }

    let d = d_sq.sqrt();

    let q = if b < 0.0 {
        n32(-0.5) * (b - d)
    } else {
        n32(-0.5) * (b + d)
    };

    match (q / a, c / a) {
    | (t0, t1) if t0 > t1 => Some((t1, t0)),
    | (t0, t1) => Some((t0, t1)),
    }
}

pub fn clamp<N: Num + num_traits::Float>(v: N, min: N, max: N) -> N {
    std::cmp::max(std::cmp::min(v, max), min)
}
