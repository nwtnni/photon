use crate::Vec3;

pub struct Ray {
    o: Vec3,
    d: Vec3,
}

impl Ray {
    #[inline(always)]
    pub fn o(&self) -> Vec3 { self.o }

    #[inline(always)]
    pub fn d(&self) -> Vec3 { self.d }

    #[inline(always)]
    pub fn at(&self, t: f32) -> Vec3 {
        self.o + self.d * t
    }
}
