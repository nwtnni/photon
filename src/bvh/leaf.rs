use crate::geometry::{Bound, Ray};
use crate::surface::{Hit, Surface};

pub const LEAF_SIZE: usize = 16;

#[derive(Copy, Clone, Debug, Default)]
pub struct Leaf<'scene>([Option<&'scene dyn Surface<'scene>>; LEAF_SIZE]);

impl<'scene> Leaf<'scene> {
    pub fn set(&mut self, index: usize, surface: &'scene dyn Surface<'scene>) {
        self.0[index] = Some(surface)
    }

    pub fn len(&self) -> usize {
        self.0.into_iter()
            .filter_map(|surface| *surface)
            .count()
    }
}

impl<'scene> Surface<'scene> for Leaf<'scene> {
    fn bound(&self) -> Bound {
        self.0.into_iter()
            .filter_map(|surface| *surface)
            .fold(Bound::smallest(), |a, b| a.union_b(&b.bound()))
    }

    fn hit(&self, ray: &mut Ray, hit: &mut Hit<'scene>) -> bool {
        let mut success = false;
        for i in 0..LEAF_SIZE {
            if let Some(surface) = self.0[i] {
                success |= surface.hit(ray, hit);
            } else {
                return success
            }
        }
        success
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        self.0.into_iter()
            .filter_map(|surface| *surface)
            .any(|surface| surface.hit_any(ray))
    }
}
