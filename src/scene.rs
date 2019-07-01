use crate::arena;
use crate::bvh;
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
    width: usize,
    height: usize,
    samples: usize,
    camera: camera::Camera,
    lights: Vec<&'scene dyn light::Light>,
    surface: bvh::Linear<'scene, &'scene dyn geom::Surface<'scene>>,
    integrator: &'scene dyn integrator::Integrator<'scene>,
}

impl<'scene> Scene<'scene> {
    pub fn new(
        arena: &'scene arena::Arena,
        width: usize,
        height: usize,
        samples: usize,
        camera: camera::Camera,
        lights: Vec<&'scene dyn light::Light>,
        surfaces: Vec<&'scene dyn geom::Surface<'scene>>,
        integrator: &'scene dyn integrator::Integrator<'scene>
    ) -> Self {
        let surface = bvh::Linear::new(arena, &surfaces);
        Scene { width, height, samples, camera, lights, surface, integrator }
    }

    pub fn lights(&self) -> &[&'scene dyn light::Light] {
        &self.lights
    }

    pub fn render(&self) {


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
