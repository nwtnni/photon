use crate::math::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
    pub min: f32,
    pub max: f32,
}

impl Ray {
    #[inline(always)]
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Ray { o, d: d.normalize(), min: 0.001, max: std::f32::MAX, }
    }

    #[inline(always)]
    pub fn at(&self, t: f32) -> Vec3 {
        self.o + self.d * t
    }
}