use crate::geometry::Ray;
use crate::surface::{Hit, Surface};

#[derive(Clone, Debug, Default)]
pub struct List<'scene> {
    surfaces: Vec<&'scene Surface<'scene>>,
}

impl<'scene> List<'scene> {
    pub fn push(&mut self, surface: &'scene Surface<'scene>) {
        self.surfaces.push(surface);
    }
}

impl<'scene> Surface<'scene> for List<'scene> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit<'scene>) -> bool {
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
