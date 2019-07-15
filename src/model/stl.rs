use std::path;
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
        ascii(stl, arena, material)
    } else {
        Binary::new(stl).parse(arena, material)
    }
}

fn ascii<'scene>(
    stl: Vec<u8>,
    arena: &'scene arena::Arena,
    material: &'scene dyn bxdf::BxDF
) -> geom::Mesh<'scene> {

    let stl = String::from_utf8(stl).expect("[INTERNAL ERROR]: invalid ASCII STL file");
    let mut tokens = stl.split_whitespace();
    let mut vs = Vec::new();
    let mut ns = Vec::new();

    macro_rules! go {
        () => {
            match tokens.next() {
            | Some(s) => s.parse::<f32>().expect("[INTERNAL ERROR]: invalid STL file"),
            | None => panic!("[INTERNAL ERROR]: invalid STL file"),
            }
        }
    }

    while let Some(token) = tokens.next() {
        if token != "facet" { continue }

        assert!(tokens.next() == Some("normal"));

        let n = math::Vec3::new(go!(), go!(), go!());
        let n = arena.alloc(n);
        ns.push(n);

        assert!(tokens.next() == Some("outer"));
        assert!(tokens.next() == Some("loop"));

        for _ in 0..3 {
            assert!(tokens.next() == Some("vertex"));
            let v = math::Vec3::new(go!(), go!(), go!());
            let v = arena.alloc(v);
            vs.push(v);
        }

        assert!(tokens.next() == Some("endloop"));
    }

    let ts = ns.into_iter()
        .enumerate()
        .map(|(i, n)| geom::Tri::new([vs[i * 3 + 0], vs[i * 3 + 1], vs[i * 3 + 2]], [n, n, n]))
        .collect::<Vec<_>>();

    geom::Mesh::new(arena, material, &ts)
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
