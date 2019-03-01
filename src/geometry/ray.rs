use crate::geometry::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    o: Vec3,
    d: Vec3,
    t: f32,
}

impl Ray {
    #[inline(always)]
    pub fn new(o: Vec3, d: Vec3, t: f32) -> Self {
        Ray { o, d, t }
    }

    #[inline(always)]
    pub fn o(&self) -> Vec3 { self.o }

    #[inline(always)]
    pub fn d(&self) -> Vec3 { self.d }

    #[inline(always)]
    pub fn t(&self) -> f32 { self.t }

    #[inline(always)]
    pub fn at(&self, t: f32) -> Vec3 {
        self.o + self.d * t
    }
}
