use crate::math::{Ray, Vec3};
use crate::geom;

#[derive(Copy, Clone, Debug)]
pub struct Translate<'scene> {
    offset: Vec3,
    surface: &'scene dyn geom::Surface<'scene>,
}

impl<'scene> Translate<'scene> {
    pub fn new(offset: Vec3, surface: &'scene dyn geom::Surface<'scene>) -> Self {
        Translate { offset, surface }
    }
}

impl<'scene> geom::Surface<'scene> for Translate<'scene> {
    fn bound(&self) -> geom::Box3 {
        let bound = self.surface.bound();
        geom::Box3::new(
            bound.min + self.offset,
            bound.max + self.offset,
        )
    }

    fn hit(&self, ray: &mut Ray, hit: &mut geom::Record<'scene>) -> bool {
        let mut offset = ray.with_origin(ray.origin - self.offset);
        if self.surface.hit(&mut offset, hit) {
            hit.p += self.offset;
            ray.set_max(offset.max);
            true
        } else {
            false
        }
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        let offset = ray.with_origin(ray.origin - self.offset);
        self.surface.hit_any(&offset)
    }
}
