use std::io;

use crate::arena;
use crate::camera;
use crate::bxdf;
use crate::geom;
use crate::integrator;
use crate::light;
use crate::math;
use crate::model;
use crate::scene;

pub struct Parser<'scene, R> {
    arena: &'scene arena::Arena,
    lexer: scene::Lexer<R>,
}

impl<'scene, R> Parser<'scene, R> where R: io::Read {
    pub fn new(arena: &'scene arena::Arena, lexer: scene::Lexer<R>) -> Self {
        Parser { arena, lexer }
    }

    pub fn parse_scene(&mut self) -> scene::Scene<'scene> {
        let mut width = 200;
        let mut height = 100;
        let mut samples = 64;
        let mut camera = camera::Camera::default();
        let mut lights = Vec::default();
        let mut surfaces = Vec::default();
        let mut integrator = &integrator::Normal as &dyn integrator::Integrator;

        while let Some(item) = self.lexer.next() {
            use scene::Token::*;
            match item {
            | Width => width = self.parse_int() as usize,
            | Height => height = self.parse_int() as usize,
            | Samples => samples = self.parse_int() as usize,
            | Camera => {
                camera = self.parse_camera();
            }
            | Light => {
                match self.parse_light() {
                | (light, Some(surface)) => {
                    lights.push(light);
                    surfaces.push(surface);
                }
                | (light, None) => {
                    lights.push(light);
                }
                }
            }
            | Surface => {
                let surface = self.parse_surface();
                surfaces.push(surface);
            }
            | Integrator => {
                integrator = self.parse_integrator();
            }
            | _ => panic!("[SCENE ERROR]: expected top-level element"),
            }
        }

        scene::Scene::new(
            self.arena,
            width,
            height,
            samples,
            camera,
            lights,
            surfaces,
            integrator
        )
    }

    fn parse_camera(&mut self) -> camera::Camera {
        let origin = self.parse_vec();
        let toward = self.parse_vec();
        let up = self.parse_vec();
        let fov = self.parse_float();
        let aspect = self.parse_float();
        let aperture = self.parse_float();
        let focus = self.parse_float();
        camera::Camera::new(origin, toward, up, fov, aspect, aperture, focus)
    }

    fn parse_integrator(&mut self) -> &'scene dyn integrator::Integrator<'scene> {
        use scene::Token::*;
        match self.lexer.next() {
        | Some(Normal) => self.arena.alloc(integrator::Normal),
        | Some(Path) => self.arena.alloc(integrator::Path),
        | Some(Light) => self.arena.alloc(integrator::Light),
        | Some(BxDF) => self.arena.alloc(integrator::BxDF),
        | Some(Point) => self.arena.alloc(integrator::Point),
        | _ => panic!("[SCENE ERROR]: expected integrator"),
        }
    }

    fn parse_light(&mut self) -> (&'scene dyn light::Light, Option<&'scene dyn geom::Surface<'scene>>) {
        use scene::Token::*;
        match self.lexer.next() {
        | Some(Point) => {
            let p = self.parse_vec();
            let i = self.parse_vec();
            (self.arena.alloc(light::Point::new(p, i)), None)
        }
        | Some(Quad) => {
            let p = self.parse_vec();
            let u = self.parse_vec();
            let v = self.parse_vec();
            let bxdf = self.parse_bxdf();
            let emit = Some(self.parse_vec());
            let quad = self.arena.alloc(geom::Quad::new(p, u, v, bxdf, emit));
            (quad, Some(quad))
        }
        | _ => panic!("[SCENE ERROR]: expected light"),
        }
    }

    fn parse_surface(&mut self) -> &'scene dyn geom::Surface<'scene> {
        use scene::Token::*;
        match self.lexer.next() {
        | Some(Sphere) => {
            let center = self.parse_vec();
            let radius = self.parse_float();
            let bxdf = self.parse_bxdf();
            self.arena.alloc(geom::Sphere::new(center, radius, bxdf))
        }
        | Some(Quad) => {
            let p = self.parse_vec();
            let u = self.parse_vec();
            let v = self.parse_vec();
            let bxdf = self.parse_bxdf();
            let emit = None;
            self.arena.alloc(geom::Quad::new(p, u, v, bxdf, emit)) 
        }
        | Some(Mesh) => {
            let path = self.parse_string();
            let bxdf = self.parse_bxdf();
            let mesh = model::obj::parse(path, &self.arena, bxdf);
            self.arena.alloc(mesh)
        }
        | _ => panic!("[SCENE ERROR]: expected surface"),
        }
    }

    fn parse_bxdf(&mut self) -> &'scene dyn bxdf::BxDF {
        use scene::Token::*;
        match self.lexer.next() {
        | Some(Glazed) => {
            let eta = self.parse_float();
            let bxdf = self.parse_bxdf();
            self.arena.alloc(bxdf::Glazed::new(bxdf, eta))
        }
        | Some(Mirror) => {
            self.arena.alloc(bxdf::Mirror)
        }
        | Some(Lambertian) => {
            let color = self.parse_vec();
            self.arena.alloc(bxdf::Lambertian::new(color))
        }
        | Some(Specular) => {
            let color = self.parse_vec();
            let eta = self.parse_float();
            self.arena.alloc(bxdf::Specular::new(color, eta))
        },
        | _ => panic!("[SCENE ERROR]: expected BxDF"),
        }
    }

    fn parse_vec(&mut self) -> math::Vec3 {
        let x = self.parse_float();
        let y = self.parse_float();
        let z = self.parse_float();
        math::Vec3::new(x, y, z)
    }

    fn parse_string(&mut self) -> String {
        match self.lexer.next() {
        | Some(scene::Token::String(s)) => s,
        | _ => panic!("[SCENE ERROR]: expected String"),
        }
    }

    fn parse_int(&mut self) -> i32 {
        match self.lexer.next() {
        | Some(scene::Token::Int(i)) => i,
        | _ => panic!("[SCENE ERROR]: expected integer"),
        }
    }

    fn parse_float(&mut self) -> f32 {
        match self.lexer.next() {
        | Some(scene::Token::Int(i)) => i as f32,
        | Some(scene::Token::Float(f)) => f,
        | _ => panic!("[SCENE ERROR]: expected float"),
        }
    }
}
