use std::ops::{
    Mul,
};
use noisy_float::prelude::*;

use crate::math::{Mat4f, Normal3f, Point3f, Vec3f};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Transform {
    m: Mat4f,
    m_inv: Mat4f,
}

impl Transform {
    pub fn inverse(&self) -> Self {
        Transform {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    pub fn transpose(&self) -> Self {
        Transform {
            m: self.m.transpose(),
            m_inv: self.m_inv.transpose(),
        }
    }

    pub fn translate<V: Into<Vec3f>>(v: V) -> Self {
        let v = v.into();
        Transform {
            m: Mat4f::translate(v),
            m_inv: Mat4f::translate(-v),
        }
    }

    pub fn scale<V: Into<Vec3f>>(v: V) -> Self {
        let v = v.into();
        Transform {
            m: Mat4f::scale(v),
            m_inv: Mat4f::scale([
                n32(1.0) / v.x(),
                n32(1.0) / v.y(),
                n32(1.0) / v.z(),
            ]),
        }
    }

    pub fn rotate_x(theta: N32) -> Self {
        let rotation = Mat4f::rotate_x(theta);
        Transform {
            m: rotation,
            m_inv: rotation.transpose(),
        }
    }

    pub fn rotate_y(theta: N32) -> Self {
        let rotation = Mat4f::rotate_y(theta);
        Transform {
            m: rotation,
            m_inv: rotation.transpose(),
        }
    }

    pub fn rotate_z(theta: N32) -> Self {
        let rotation = Mat4f::rotate_z(theta);
        Transform {
            m: rotation,
            m_inv: rotation.transpose(),
        }
    }

    pub fn rotate<V: Into<Vec3f>>(theta: N32, axis: V) -> Self {
        let axis = axis.into();
        let rotation = Mat4f::rotate(theta, axis);
        Transform {
            m: rotation,
            m_inv: rotation.transpose(),
        }
    }

    pub fn look_at(pos: Point3f, look: Point3f, up: Vec3f) -> Self {
        let look = Mat4f::look_at(pos, look, up);
        Transform {
            m: look,
            m_inv: look.inverse(),
        }
    }
}

impl_all!(impl_mul_tv, Point3f, Transform, Point3f);
impl_all!(impl_mul_tv, Vec3f, Transform, Vec3f);
impl_all!(impl_mul_tn, Normal3f, Transform, Normal3f);
