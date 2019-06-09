use crate::arena;
use crate::bvh;
use crate::camera;
use crate::geom;
use crate::light;
use crate::math;

#[derive(Debug)]
pub struct Scene<'scene> {
    camera: camera::Camera,
    lights: Vec<&'scene dyn light::Light>,
    surface: &'scene dyn geom::Surface<'scene>,
}

impl<'scene> Scene<'scene> {
    pub fn new(camera: camera::Camera, lights: Vec<&'scene dyn light::Light>, surface: &'scene dyn geom::Surface<'scene>) -> Self {
        Scene { camera, lights, surface }
    }

    pub fn lights(&self) -> &[&'scene dyn light::Light] {
        &self.lights
    }
}

impl<'scene> geom::Surface<'scene> for Scene<'scene> {
    fn bound(&self) -> geom::Bound {
        self.surface.bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        self.surface.hit(ray, hit)
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.surface.hit_any(ray)
    }
}
