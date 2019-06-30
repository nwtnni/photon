use crate::prelude::*;
use crate::math;
use crate::geom;

pub const LEAF_SIZE: usize = 16;

#[derive(Copy, Clone, Debug)]
pub struct Leaf<'scene, S> {
    bound: geom::Box3,
    surfaces: [Option<&'scene S>; LEAF_SIZE],
}

impl<'scene, S> Leaf<'scene, S> where S: Surface<'scene> {
    pub fn set(&mut self, index: usize, surface: &'scene S) {
        self.bound = self.bound.union_b(&surface.bound());
        self.surfaces[index] = Some(surface);
    }

    pub fn len(&self) -> usize {
        self.surfaces.into_iter()
            .filter_map(|surface| *surface)
            .count()
    }
}

impl<'scene, S> Surface<'scene> for Leaf<'scene, S> where S: Surface<'scene> {
    fn bound(&self) -> geom::Box3 {
        self.bound
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        let mut success = false;
        for i in 0..LEAF_SIZE {
            if let Some(surface) = self.surfaces[i] {
                success |= surface.hit(ray, hit);
            } else {
                return success
            }
        }
        success
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.surfaces.into_iter()
            .filter_map(|surface| *surface)
            .any(|surface| surface.hit_any(ray))
    }
}

impl<'scene, S> Default for Leaf<'scene, S> {
    fn default() -> Self {
        Leaf {
            bound: geom::Box3::smallest(),
            surfaces: [None; LEAF_SIZE],
        }
    }
}
