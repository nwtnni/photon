use crate::geometry::{Bound, Ray};
use crate::surface::{Hit, Surface};

/// Naive list of surfaces.
/// Intersection is linear time w.r.t number of surfaces.
#[derive(Clone, Debug, Default)]
pub struct List<'scene> {
    surfaces: Vec<&'scene dyn Surface<'scene>>,
}

impl<'scene> From<Vec<&'scene dyn Surface<'scene>>> for List<'scene> {
    fn from(surfaces: Vec<&'scene dyn Surface<'scene>>) -> Self {
        List { surfaces }
    }
}

impl<'scene> List<'scene> {
    /// Construct a list with the provided capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        List { surfaces: Vec::with_capacity(capacity) }
    }

    /// Append a new surface to this list.
    pub fn push(&mut self, surface: &'scene dyn Surface<'scene>) {
        self.surfaces.push(surface);
    }
}

impl<'scene> Surface<'scene> for List<'scene> {
    fn bound(&self) -> Bound {
        self.surfaces.iter()
            .map(|s| s.bound())
            .fold(Bound::smallest(), |a, b| a.union_b(&b))
    }

    fn hit(&self, ray: &mut Ray, hit: &mut Hit<'scene>) -> bool {
        if cfg!(feature = "stats") {
            crate::stats::INTERSECTION_TESTS.inc();
            crate::stats::LIST_INTERSECTION_TESTS.inc();
        }
        let mut success = false;
        for surface in &self.surfaces {
            if surface.hit(ray, hit) {
                success = true;
            }
        }
        success
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        if cfg!(feature = "stats") {
            crate::stats::INTERSECTION_TESTS.inc();
            crate::stats::LIST_INTERSECTION_TESTS.inc();
        }
        for surface in &self.surfaces {
            if surface.hit_any(ray) { return true }
        }
        false
    }
}
