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
        binary(stl, arena, material)
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

fn binary<'scene>(
    stl: Vec<u8>,
    arena: &'scene arena::Arena,
    material: &'scene dyn bxdf::BxDF
) -> geom::Mesh<'scene> {

    let mut cursor = 80;
    let mut ts = Vec::new();

    let count = LE::read_u32(&stl[cursor..]);      
    cursor += 4;

    macro_rules! float {
        () => {{
            let f = LE::read_f32(&stl[cursor..]);
            cursor += 4;
            f
        }}
    }

    macro_rules! vec3 {
        () => {{
            let v = math::Vec3::new(float!(), float!(), float!());
            let v = arena.alloc(v);
            v
        }}
    }

    for _ in 0..count {
        let n = vec3!();
        let a = vec3!();
        let b = vec3!();
        let c = vec3!();
        ts.push(geom::Tri::new([a, b, c], [n, n, n]));
        cursor += 2;
    }

    println!("{:?}", ts);

    geom::Mesh::new(arena, material, &ts)
}
