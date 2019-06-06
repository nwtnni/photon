use crate::geom::{Bound, Ray, Vec3};
use crate::surface;

#[derive(Copy, Clone, Debug)]
pub struct Translate<'scene> {
    offset: Vec3,
    surface: &'scene dyn surface::Surface<'scene>,
}

impl<'scene> Translate<'scene> {
    pub fn new(offset: Vec3, surface: &'scene dyn surface::Surface<'scene>) -> Self {
        Translate { offset, surface }
    }
}

impl<'scene> surface::Surface<'scene> for Translate<'scene> {
    fn bound(&self) -> Bound {
        let bound = self.surface.bound();
        Bound::new(
            bound.min() + self.offset,
            bound.max() + self.offset,
        )
    }

    fn hit(&self, ray: &mut Ray, hit: &mut surface::Record<'scene>) -> bool {
        let mut offset = Ray { o: ray.o - self.offset, .. *ray };
        if self.surface.hit(&mut offset, hit) {
            hit.p += self.offset;
            ray.max = offset.max;
            true
        } else {
            false
        }
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        let offset = Ray { o: ray.o - self.offset, .. *ray };
        self.surface.hit_any(&offset)
    }
}
