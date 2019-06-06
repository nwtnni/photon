use crate::geom::Vec3;
use crate::texture::Texture;

#[derive(Copy, Clone, Debug)]
pub struct Constant(Vec3);

impl Constant {
    pub fn new(color: Vec3) -> Self {
        Constant(color)
    }
}

impl Texture for Constant {
    fn evaluate(&self, _: f32, _: f32) -> Vec3 {
        self.0
    }
}
