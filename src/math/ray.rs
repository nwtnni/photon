use noisy_float::prelude::*;

use crate::math::{Point3f, Vec3f};

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    o: Point3f,
    d: Vec3f,
    t_min: N32,
    t_max: N32,
}

impl Ray {
    #[inline]
    pub fn o(&self) -> Point3f { self.o }

    #[inline]
    pub fn d(&self) -> Vec3f { self.d }

    #[inline]
    pub fn t_min(&self) -> N32 { self.t_min }

    #[inline]
    pub fn t_max(&self) -> N32 { self.t_max }

    #[inline]
    pub fn new(o: Point3f, d: Vec3f) -> Self {
        Ray {
            o,
            d,
            t_min: n32(0.0),
            t_max: n32(std::f32::INFINITY),
        }
    }
}
