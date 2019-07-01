use crate::camera;
use crate::geom;
use crate::light;
use crate::math;

mod token;
mod lexer;
mod parser;

pub use token::Token;
pub use lexer::Lexer;

#[derive(Debug)]
pub struct Scene<'scene> {
    background: math::Vec3,
    camera: camera::Camera,
    lights: &'scene [&'scene dyn light::Light],
    surface: &'scene dyn geom::Surface<'scene>,
}

impl<'scene> Scene<'scene> {
    pub fn new(
        background: math::Vec3, 
        camera: camera::Camera,
        lights: &'scene [&'scene dyn light::Light],
        surface: &'scene dyn geom::Surface<'scene>
    ) -> Self {
        Scene { background, camera, lights, surface }
    }

    pub fn background(&self) -> math::Vec3 {
        self.background
    }

    pub fn lights(&self) -> &[&'scene dyn light::Light] {
        &self.lights
    }
}

impl<'scene> geom::Surface<'scene> for Scene<'scene> {
    fn bound(&self) -> geom::Box3 {
        self.surface.bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        self.surface.hit(ray, hit)
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.surface.hit_any(ray)
    }
}
