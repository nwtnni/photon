use crate::arena::Arena;
use crate::material::Material;
use crate::geometry::{Mesh, Tri, Vec3};
use crate::surface::Surface;

pub fn parse<'scene, P>(
    obj: P,
    arena: &'scene Arena,
    material: &'scene dyn Material,
    t_min: f32,
    t_max: f32,
) -> Mesh<'scene>
    where P: AsRef<std::path::Path>,
{
    let obj = std::fs::read_to_string(obj).expect("[INTERNAL ERROR]: could not read OBJ file");
    let mut fs = Vec::new();
    let mut vs = Vec::new();
    let mut ns = Vec::new();

    for line in obj.lines() {

        let mut iter = line.trim_end()
            .split_whitespace();

        macro_rules! go {
            ($type:ty) => {
                match iter.next() {
                | Some(s) => s.parse::<$type>().expect("[INTERNAL ERROR]: invalid OBJ file"),
                | None => panic!("[INTERNAL ERROR]: invalid OBJ file"),
                }
            }
        }

        match iter.next() {
        | Some("v") => {
            let vertex = Vec3::new(go!(f32), go!(f32), go!(f32));
            let vertex = arena.alloc(vertex);
            vs.push(vertex);
            ns.push(Vec3::default());
        }
        | Some("f") => {
            fs.push((go!(usize) - 1, go!(usize) - 1, go!(usize) - 1));
        }
        | _ => continue,
        }
    }

    for &(a, b, c) in &fs {
        let n = (vs[b] - vs[a]).cross(&(vs[c] - vs[a])).normalize();
        ns[a] += n;
        ns[b] += n;
        ns[c] += n;
    }

    let ns = ns.into_iter()
        .map(|n| n.normalize())
        .map(|n| arena.alloc(n))
        .collect::<Vec<_>>();

    let ts = fs.into_iter()
        .map(|(a, b, c)| {
            arena.alloc(Tri::new([vs[a], vs[b], vs[c]], [ns[a], ns[b], ns[c]])) as &'scene dyn Surface
        })
        .collect::<Vec<_>>();

    Mesh::new(arena, material, &ts, t_min, t_max)
}
