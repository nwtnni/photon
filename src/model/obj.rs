use crate::arena::CopyArena;
use crate::geometry::{Tri, Vec3};

pub fn parse<'scene, P>(arena: &'scene CopyArena, obj: P) -> Vec<Tri<'scene>>
    where P: AsRef<std::path::Path>,
{
    let obj = std::fs::read_to_string(obj).expect("[INTERNAL ERROR]: could not read OBJ file");
    let mut fs = Vec::new();
    let mut vs = Vec::new();

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
        | Some("v") => vs.push(arena.alloc(Vec3::new(go!(f32), go!(f32), go!(f32)))),
        | Some("f") => fs.push(Tri::new(vs[go!(usize) - 1], vs[go!(usize) - 1], vs[go!(usize) - 1])),
        | _         => continue,
        }
    }

    fs
}
