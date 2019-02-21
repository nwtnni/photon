use crate::Ray;
use crate::geometry::{Hit, Surface};

#[derive(Clone, Debug, Default)]
pub struct List<'arena> {
    surfaces: Vec<&'arena Surface>,
}

impl<'arena> List<'arena> {
    pub fn push(&mut self, surface: &'arena Surface) {
        self.surfaces.push(surface);
    }
}

impl<'arena> Surface for List<'arena> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let mut record = Hit::default();
        let mut closest = t_max;
        let mut success = false;
        for surface in &self.surfaces {
            if surface.hit(ray, t_min, closest, &mut record) {
                success = true;
                closest = record.t;
                *hit = record;
            }
        }
        success
    }
}
