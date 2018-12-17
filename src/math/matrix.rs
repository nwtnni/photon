use noisy_float::prelude::*;
use num_traits::real::Real;

use crate::math::{Num, Vec3};

pub type Mat4f = Mat4<N32>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Mat4<N>([N; 16]);

impl<N: Num> Default for Mat4<N> {
    fn default() -> Self {
        let o = N::one();
        let z = N::zero();
        Mat4([
            o, z, z, z,
            z, o, z, z,
            z, z, o, z,
            z, z, z, o,
        ])
    }
}

impl<N: Num> Mat4<N> {
    pub fn identity() -> Self {
        Self::default()
    }

    pub fn translate<V: Into<Vec3<N>>>(v: V) -> Self {
        let o = N::one();
        let z = N::zero();
        let v = v.into();
        Mat4([
            o, z, z, v.x(),
            z, o, z, v.y(),
            z, z, o, v.z(),
            z, z, z, o,
        ])
    }
    
    pub fn scale<V: Into<Vec3<N>>>(v: V) -> Self {
        let o = N::one();
        let z = N::zero();
        let v = v.into();
        Mat4([
            v.x(), z, z, z,
            z, v.y(), z, z,
            z, z, v.z(), z,
            z, z, z, o,
        ])
    }
}

impl<N: Num + Real> Mat4<N> {
    pub fn rotate_x(theta: N) -> Self {
        let o = N::one(); 
        let z = N::zero();
        Mat4([
            o, z, z, z,
            z, theta.cos(), -theta.sin(), z,
            z, theta.sin(), theta.cos(), z,
            z, z, z, o,
        ])
    }

    pub fn rotate_y(theta: N) -> Self {
        let o = N::one(); 
        let z = N::zero();
        Mat4([
            theta.cos(), z, theta.sin(), z,
            z, o, z, z,
            -theta.sin(), z, theta.cos(), z,
            z, z, z, o,
        ])
    }

    pub fn rotate_z(theta: N) -> Self {
        let o = N::one(); 
        let z = N::zero();
        Mat4([
            theta.cos(), -theta.sin(), z, z,
            theta.sin(), theta.cos(), z, z,
            z, z, o, z,
            z, z, z, o,
        ])
    }
}
