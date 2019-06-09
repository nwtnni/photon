use crate::math::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub o: Vec3,
    pub d: Vec3,
    pub min: f32,
    pub max: f32,
    pub inv: [f32; 3],
    pub sign: [usize; 3],
}

impl Ray {
    #[inline(always)]
    pub fn new(o: Vec3, d: Vec3) -> Self {
        let d = d.normalize();
        Ray {
            o,
            d,
            min: 0.001,
            max: std::f32::MAX,
            inv: [1.0 / d.x(), 1.0 / d.y(), 1.0 / d.z()],
            sign: [
                (d.x() < 0.0) as usize,
                (d.y() < 0.0) as usize,
                (d.z() < 0.0) as usize,
            ],
        }
    }

    #[inline(always)]
    pub fn at(&self, t: f32) -> Vec3 {
        self.o + self.d * t
    }
}
