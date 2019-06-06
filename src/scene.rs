use crate::arena;
use crate::bvh;
use crate::camera;
use crate::geom::{Bound, Ray};
use crate::light;
use crate::surface;

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

impl<'scene> surface::Surface<'scene> for Scene<'scene> {
    fn bound(&self) -> Bound {
        self.surfaces.bound()
    }

    fn hit(&self, ray: &mut Ray, hit: &mut surface::Record<'scene>) -> bool {
        self.surfaces.hit(ray, hit)
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        self.surfaces.hit_any(ray)
    }
}
