use std::ops::{Mul, Index, IndexMut};

use noisy_float::prelude::*;
use num_traits::real::Real;

use crate::math::{Num, Point3, Vec3};

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
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> N {
        self.0[i * 4 + j]            
    }

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

    pub fn transpose(&self) -> Self {
        Mat4([
             self.0[0], self.0[4], self.0[8], self.0[12],
             self.0[1], self.0[5], self.0[9], self.0[13],
             self.0[2], self.0[6], self.0[10], self.0[14],
             self.0[3], self.0[7], self.0[11], self.0[15],
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

    // See https://github.com/mmp/pbrt-v3/blob/af4b70601bb770caa720c569f1641c4ddff333b7/src/core/transform.cpp#L179-L201
    pub fn rotate<V: Into<Vec3<N>>>(theta: N, axis: V) -> Self {
        let o = N::one();
        let z = N::zero();
        let a = axis.into().normalize();
        let sin = theta.sin();
        let cos = theta.cos();
        Mat4([
             // Rotate around first basis vector
            a.x() * a.x() + cos * (o - a.x() * a.x()),
            (o - cos) * a.x() * a.y() - sin * a.z(),
            (o - cos) * a.x() * a.z() + sin * a.y(),
            z,

            (o - cos) * a.x() * a.y() + sin * a.z(),
            a.y() * a.y() + cos * (o - a.y() * a.y()),
            (o - cos) * a.y() * a.z() - sin * a.x(),
            z,

            (o - cos) * a.x() * a.z() - sin * a.y(),
            (o - cos) * a.y() * a.z() + sin * a.x(),
            a.z() * a.z() + cos * (o - a.z() * a.z()),
            z,

            z, z, z, o,
        ])
    }

    pub fn look_at(pos: Point3<N>, look: Point3<N>, up: Vec3<N>) -> Self {
        let o = N::one();
        let z = N::zero();
        let dir = (look - pos).normalize();
        let right = up.cross_v(&dir).normalize();
        let up = dir.cross_v(&right);
        Mat4([
            right.x(), up.x(), dir.x(), pos.x(),
            right.y(), up.y(), dir.y(), pos.y(),
            right.z(), up.z(), dir.z(), pos.z(),
            z, z, z, o,
        ])
    }
}

impl<N> Index<usize> for Mat4<N> {
    type Output = N;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<N> IndexMut<usize> for Mat4<N> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl_all!(impl_mp, Point3<N>, Mat4<N>, Point3<N>);
impl_all!(impl_mv, Vec3<N>, Mat4<N>, Vec3<N>);
