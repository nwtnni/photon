#[macro_use]
mod macros;
mod bounds;
mod normal;
mod point;
mod ray;
mod vector;

pub use self::normal::*;
pub use self::point::*;
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
