use crate::math::Vec3;

#[readonly::make]
#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub min: f32,
    pub max: f32,
    pub inv: [f32; 3],
    pub sign: [usize; 3],
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        let dir = dir.normalize();
        Ray {
            origin,
            dir,
            min: 0.001,
            max: std::f32::MAX,
            inv: [1.0 / dir.x(), 1.0 / dir.y(), 1.0 / dir.z()],
            sign: [
                (dir.x() < 0.0) as usize,
                (dir.y() < 0.0) as usize,
                (dir.z() < 0.0) as usize,
            ],
        }
    }

    pub fn with_origin(&self, origin: Vec3) -> Self {
        Ray { origin, .. *self }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }

    pub fn set_max(&mut self, t: f32) {
        self.max = t
    }
}
