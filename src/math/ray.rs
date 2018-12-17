use noisy_float::prelude::*;

use crate::math::{Point3f, Vec3f};

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    o: Point3f,
    d: Vec3f,
}

impl Ray {
    #[inline]
    pub fn o(&self) -> Point3f { self.o }

    #[inline]
    pub fn d(&self) -> Vec3f { self.d }

    #[inline]
    pub fn new(o: Point3f, d: Vec3f) -> Self {
        Ray { o, d }
    }
}
