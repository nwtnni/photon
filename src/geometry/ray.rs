use noisy_float::prelude::*;

use crate::geometry::{Point3f, Vector3f};

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    o: Point3f,
    d: Vector3f,
}

impl Ray {
    #[inline]
    pub fn o(&self) -> Point3f { self.o }

    #[inline]
    pub fn d(&self) -> Vector3f { self.d }

    #[inline]
    pub fn new(o: Point3f, d: Vector3f) -> Self {
        Ray { o, d }
    }
}
