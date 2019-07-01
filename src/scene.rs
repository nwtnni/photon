use crate::camera;
use crate::geom;
use crate::light;
use crate::integrator;
use crate::math;

mod token;
mod lexer;
mod parser;

pub use token::Token;
pub use lexer::Lexer;
pub use parser::Parser;

#[derive(Debug)]
pub struct Scene<'scene> {
    camera: camera::Camera,
    lights: Vec<&'scene dyn light::Light>,
    surfaces: Vec<&'scene dyn geom::Surface<'scene>>,
    integrator: &'scene dyn integrator::Integrator<'scene>,
}

impl<'scene> Scene<'scene> {
    pub fn new(
        camera: camera::Camera,
        lights: Vec<&'scene dyn light::Light>,
        surfaces: Vec<&'scene dyn geom::Surface<'scene>>,
        integrator: &'scene dyn integrator::Integrator<'scene>
    ) -> Self {
        Scene { camera, lights, surfaces, integrator }
    }

    pub fn lights(&self) -> &[&'scene dyn light::Light] {
        &self.lights
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
