use std::path;
use std::fs;

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
    let stl = fs::read_to_string(stl)
        .expect("[INTERNAL ERROR]: could not read STL file");

    if stl.starts_with("solid") {
        ascii(stl, arena, material)
    } else {
        binary(stl, arena, material)
    }
}

fn ascii<'scene>(
    stl: String,
    arena: &'scene arena::Arena,
    material: &'scene dyn bxdf::BxDF
) -> geom::Mesh<'scene> {

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
    stl: String,
    arena: &'scene arena::Arena,
    material: &'scene dyn bxdf::BxDF
) -> geom::Mesh<'scene> {
    unimplemented!()
}
