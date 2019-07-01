use std::io;
use std::path;
use std::fs;

use crate::arena;
use crate::bxdf;
use crate::math;
use crate::scene;

pub struct Parser<'scene, R> {
    arena: &'scene arena::Arena,
    lexer: scene::Lexer<R>,
}

impl<'scene, R> Parser<'scene, R> where R: io::Read {
    pub fn new(arena: &'scene arena::Arena, lexer: scene::Lexer<R>) -> Self {
        Parser { arena, lexer }
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
        | _ => panic!("[SCENE ERROR]: expected BXDF"),
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

    fn parse_float(&mut self) -> f32 {
        match self.lexer.next() {
        | Some(scene::Token::Float(f)) => f,
        | _ => panic!("[SCENE ERROR]: expected String"),
        }
    }
}
