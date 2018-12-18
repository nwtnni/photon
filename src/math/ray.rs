use noisy_float::prelude::*;

use crate::math::{Point3f, Vec3f};

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    o: Point3f,
    d: Vec3f,
    min: N32,
    max: N32,
}

impl Ray {
    #[inline]
    pub fn o(&self) -> Point3f { self.o }

    #[inline]
    pub fn d(&self) -> Vec3f { self.d }

    #[inline]
    pub fn min(&self) -> N32 { self.min }

    #[inline]
    pub fn max(&self) -> N32 { self.max }

    #[inline]
    pub fn new(o: Point3f, d: Vec3f) -> Self {
        Ray {
            o,
            d,
            min: n32(0.0),
            max: n32(std::f32::INFINITY),
        }
    }
}
