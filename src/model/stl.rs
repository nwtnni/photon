use std::path;
use std::str;
use std::fs;

use byteorder::ByteOrder;
use byteorder::LE;

use crate::arena;
use crate::bxdf;
use crate::geom;
use crate::math;

pub fn parse<'scene, P>(
    stl: P,
    arena: &'scene arena::Arena,
    material: &'scene dyn bxdf::BxDF,
) -> geom::Mesh<'scene>
    where P: AsRef<path::Path>
{
    let stl = fs::read(stl).expect("[INTERNAL ERROR]: could not read STL file");
    if &stl[0..5] == "solid".as_bytes() {
        str::from_utf8(&stl)
            .map(ASCII::new)
            .expect("[STL ERROR]: invalid ASCII STL file")
            .parse(arena, material)
    } else {
        Binary::new(stl)
            .parse(arena, material)
    }
}

/// ASCII STL parser. Panics on malformed input.
struct ASCII<'str>(str::SplitWhitespace<'str>);

impl<'str> ASCII<'str> {
    fn new(string: &'str str) -> Self {
        ASCII(string.split_whitespace()) 
    }

    fn parse<'scene>(
        mut self,
        arena: &'scene arena::Arena,
        material: &'scene dyn bxdf::BxDF
    ) -> geom::Mesh<'scene> {
        let mut ts = Vec::new();
        while let Some(token) = self.0.next() {
            if token != "facet" { continue }
            ts.push(self.parse_tri(arena));
        }
        geom::Mesh::new(arena, material, &ts)
    }

    fn parse_tri<'scene>(&mut self, arena: &'scene arena::Arena) -> geom::Tri<'scene> {
        self.verify("normal");
        let n = arena.alloc(self.parse_vec3());
        self.verify("outer");
        self.verify("loop");
        self.verify("vertex");
        let a = arena.alloc(self.parse_vec3());
        self.verify("vertex");
        let b = arena.alloc(self.parse_vec3());
        self.verify("vertex");
        let c = arena.alloc(self.parse_vec3());
        self.verify("endloop");
        geom::Tri::new([a, b, c], [n, n, n])
    }

    fn parse_vec3(&mut self) -> math::Vec3 {
        let x = self.parse_f32(); 
        let y = self.parse_f32(); 
        let z = self.parse_f32(); 
        math::Vec3::new(x, y, z)
    }

    fn parse_f32(&mut self) -> f32 {
        self.0.next()
            .and_then(|s| s.parse::<f32>().ok())
            .expect("[STL ERROR]: invalid STL file")
    }

    fn verify(&mut self, tag: &'static str) {
        assert!(self.0.next() == Some(tag))
    }
}

/// Binary STL parser. Panics on malformed input.
struct Binary {
    buffer: Vec<u8>,
    cursor: usize,
}

impl Binary {
    fn new(buffer: Vec<u8>) -> Self {
        Binary {
            buffer,
            cursor: 76,
        }
    }

    fn parse_u32(&mut self) -> u32 {
        self.cursor += 4;
        LE::read_u32(&self.buffer[self.cursor..])
    }

    fn parse_f32(&mut self) -> f32 {
        self.cursor += 4;
        LE::read_f32(&self.buffer[self.cursor..])
    }

    fn parse_vec3(&mut self) -> math::Vec3 {
        let x = self.parse_f32();
        let y = self.parse_f32();
        let z = self.parse_f32();
        math::Vec3::new(x, y, z)
    }

    fn parse<'scene>(
        mut self,
        arena: &'scene arena::Arena,
        material: &'scene dyn bxdf::BxDF
    ) -> geom::Mesh<'scene> {

        let mut ts = Vec::new();
        let count = self.parse_u32();
        for _ in 0..count {
            let n = arena.alloc(self.parse_vec3());
            let a = arena.alloc(self.parse_vec3());
            let b = arena.alloc(self.parse_vec3());
            let c = arena.alloc(self.parse_vec3());
            ts.push(geom::Tri::new([a, b, c], [n, n, n]));
            self.cursor += 2;
        }

        geom::Mesh::new(arena, material, &ts)
    }
}
