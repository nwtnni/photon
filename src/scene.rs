use crate::arena;
use crate::bvh;
use crate::camera;
use crate::math::{Bound, Ray};
use crate::light;
use crate::geom;

#[derive(Debug)]
pub struct Scene<'scene> {
    camera: camera::Camera,
    lights: Vec<&'scene dyn light::Light>,
    surfaces: bvh::Linear<'scene>,
}

impl<'scene> Scene<'scene> {
    pub fn new(arena: &arena::Arena) -> Self {
        unimplemented!()
    }
}

impl<'scene> geom::Surface<'scene> for Scene<'scene> {
    fn bound(&self) -> Bound {
        self.surfaces.bound()
    }

    fn hit(&self, ray: &mut Ray, hit: &mut geom::Record<'scene>) -> bool {
        self.surfaces.hit(ray, hit)
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        self.surfaces.hit_any(ray)
    }
}
