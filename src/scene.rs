use crate::camera;
use crate::geom;
use crate::light;
use crate::math;

mod token;
mod lexer;
mod parser;

pub use token::Token;
pub use lexer::Lexer;
pub use parser::Parser;

#[derive(Debug, Default)]
pub struct Scene<'scene> {
    background: math::Vec3,
    camera: camera::Camera,
    lights: Vec<&'scene dyn light::Light>,
    surfaces: Vec<&'scene dyn geom::Surface<'scene>>,
}

impl<'scene> Scene<'scene> {
    pub fn set_camera(&mut self, camera: camera::Camera) {
        self.camera = camera;
    }

    pub fn push_light(&mut self, light: &'scene dyn light::Light) {
        self.lights.push(light);
    }

    pub fn push_surface(&mut self, surface: &'scene dyn geom::Surface<'scene>) {
        self.surfaces.push(surface);
    }

    pub fn background(&self) -> math::Vec3 {
        self.background
    }

    pub fn lights(&self) -> &[&'scene dyn light::Light] {
        unimplemented!()
    }
}

impl<'scene> geom::Surface<'scene> for Scene<'scene> {
    fn bound(&self) -> geom::Box3 {
        unimplemented!()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        unimplemented!()
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        unimplemented!()
    }
}
