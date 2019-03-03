use crate::geometry::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Tri<'scene>([&'scene Vec3; 3]);

impl<'scene> Tri<'scene> {
    pub fn new(a: &'scene Vec3, b: &'scene Vec3, c: &'scene Vec3) -> Self {
        Tri([a, b, c])
    }
}
