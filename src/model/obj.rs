use crate::arena;
use crate::bxdf;
use crate::math::Vec3;
use crate::geom;

pub fn parse<'scene, P>(
    obj: P,
    arena: &'scene arena::Arena,
    material: &'scene bxdf::Any<'scene>,
) -> geom::Mesh<'scene>
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
            geom::Tri::new([vs[a], vs[b], vs[c]], [ns[a], ns[b], ns[c]])
        })
        .collect::<Vec<_>>();

    geom::Mesh::new(arena, material, &ts)
}
