use crate::geometry::{Bound, Ray};
use crate::surface::{Hit, Surface};

/// Naive list of surfaces.
/// Intersection is linear time w.r.t number of surfaces.
#[derive(Clone, Debug, Default)]
pub struct List<'scene> {
    surfaces: Vec<&'scene dyn Surface<'scene>>,
}

impl<'scene> List<'scene> {
    /// Append a new surface to this list.
    pub fn push(&mut self, surface: &'scene Surface<'scene>) {
        self.surfaces.push(surface);
    }
}

impl<'scene> Surface<'scene> for List<'scene> {
    fn bound(&self, t0: f32, t1: f32) -> Bound {
        let mut bound = Bound::default();
        for surface in &self.surfaces {
            bound = bound.union(&surface.bound(t0, t1));
        }
        bound
    }

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
