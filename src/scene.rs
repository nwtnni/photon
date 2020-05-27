use std::io;
use std::path;
use std::fs;

use rayon::prelude::*;

use crate::prelude::*;
use crate::arena;
use crate::bvh;
use crate::camera;
use crate::geom;
use crate::light;
use crate::integrator;
use crate::math;
use crate::scene;
use crate::stats;

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
    surface: bvh::Tree<'scene, &'scene dyn geom::Surface<'scene>>,
    integrator: &'scene dyn integrator::Integrator<'scene>,
}

impl<'scene> Scene<'scene> {
    pub fn load<P: AsRef<path::Path>>(arena: &'scene arena::Arena, path: P) -> io::Result<Self> {
        let file = fs::File::open(path)?;
        let lexer = scene::Lexer::new(file);
        let mut parser = scene::Parser::new(arena, lexer);
        Ok(parser.parse_scene())
    }

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
        let surface = bvh::Tree::new(arena, &surfaces);
        Scene { width, height, samples, camera, lights, surface, integrator }
    }

    pub fn lights(&self) -> &[&'scene dyn light::Light] {
        &self.lights
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn render(&self) {
        let mut buffer = vec![0; self.width * self.height * 3];
        buffer.par_chunks_mut(self.width * 3)
            .enumerate()
            .map(|(y, row)| (self.height - y - 1, row))
            .for_each(|(y, row)| {
                let mut hit = geom::Hit::default();
                for x in 0..self.width {
                    let mut c = math::Vec3::default();
                    for _ in 0..self.samples {
                        let u = (x as f32 + rand::random::<f32>()) / self.width as f32;
                        let v = (y as f32 + rand::random::<f32>()) / self.height as f32;
                        let mut r = self.camera.get(u, v);
                        if self.hit(&mut r, &mut hit) {
                            c += self.integrator.shade(self, &r, &hit, 0);
                        }
                    }
                    c /= self.samples as f32;
                    row[x * 3 + 0] = (math::clamp(c[0].sqrt(), 0.0, 1.0) * 255.99) as u8;
                    row[x * 3 + 1] = (math::clamp(c[1].sqrt(), 0.0, 1.0) * 255.99) as u8;
                    row[x * 3 + 2] = (math::clamp(c[2].sqrt(), 0.0, 1.0) * 255.99) as u8;
                    stats::PIXELS_RENDERED.inc();
                }
            });

        lodepng::encode24_file("out.png", &buffer, self.width, self.height).ok();
    }
}

impl<'scene> geom::Surface<'scene> for Scene<'scene> {
    fn bound(&self) -> geom::Box3 {
        self.surface.bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Hit<'scene>) -> bool {
        self.surface.hit(ray, hit)
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.surface.hit_any(ray)
    }
}
