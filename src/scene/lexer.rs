use std::io;

use crate::scene;

pub struct Lexer<R>  {
    buffer: String,
    stream: io::Bytes<R>,
    next: Option<char>,
}

impl<R> Lexer<R> where R: io::Read {
    pub fn new(stream: R) -> Self {
        let mut stream = stream.bytes();
        let next = stream.next()
            .map(Result::unwrap)
            .map(From::from);
        Lexer {
            buffer: String::default(),
            stream,
            next,
        }
    }

    fn skip(&mut self) {
        self.next = self.stream.next()
            .map(Result::unwrap)
            .map(From::from);
    }

    fn fill(&mut self) {
        self.buffer.clear();
        while let Some(c) = self.next {
            if c.is_whitespace() { break }
            self.buffer.push(c);
            self.skip();
        }
    }
}

impl<R> Iterator for Lexer<R> where R: io::Read {
    type Item = scene::Token;
    fn next(&mut self) -> Option<Self::Item> {

        while let Some(c) = self.next {

            if c.is_whitespace() {
                self.skip(); 
                continue
            }

            self.fill();      
            
            let token = if c.is_digit(10) {
                self.buffer.parse::<f32>()
                    .map(scene::Token::Float)
                    .expect("[SCENE ERROR]: invalid float")
            } else {
                use scene::Token::*;
                match self.buffer.as_ref() {
                | "camera" => Camera,
                | "integrator" => Integrator,
                | "surface" => Surface,
                | "light" => Light,
                | "bxdf" => BxDF,
                | "sphere" => Sphere,
                | "quad" => Quad,
                | "point" => Point,
                | "mesh" => Mesh,
                | "glazed" => Glazed,
                | "mirror" => Mirror,
                | "lambertian" => Lambertian,
                | "specular" => Specular,
                | _ => String(self.buffer.clone()),
                }
            };

            return Some(token)
        }
        None
    }
}
