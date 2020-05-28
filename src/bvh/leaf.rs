use crate::prelude::*;
use crate::math;
use crate::geom;

pub const LEAF_SIZE: usize = 16;

#[derive(Copy, Clone, Debug)]
pub struct Leaf<S> {
    bound: geom::Box3,
    surfaces: [Option<S>; LEAF_SIZE],
}

impl<'scene, S> Leaf<S> where S: Surface<'scene> {
    pub fn set(&mut self, index: usize, surface: S) {
        self.bound = self.bound.union_b(&surface.bound());
        self.surfaces[index] = Some(surface);
    }

    pub fn len(&self) -> usize {
        self.surfaces
            .iter()
            .filter_map(|surface| surface.as_ref())
            .count()
    }
}

impl<'scene, S> Surface<'scene> for Leaf<S> where S: Surface<'scene> {
    fn bound(&self) -> geom::Box3 {
        self.bound
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Hit<'scene>) -> bool {
        let mut success = false;
        for i in 0..LEAF_SIZE {
            if let Some(surface) = &self.surfaces[i] {
                success |= surface.hit(ray, hit);
            } else {
                return success
            }
        }
        success
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        for i in 0..LEAF_SIZE {
            if let Some(surface) = &self.surfaces[i] {
                if surface.hit_any(ray) { return true }
            } else {
                return false
            }
        }
        false
    }
}

impl<S> Default for Leaf<S> {
    fn default() -> Self {
        Leaf {
            bound: Default::default(),
            surfaces: Default::default(),
        }
    }
}
