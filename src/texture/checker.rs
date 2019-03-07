use crate::geometry::Vec3;
use crate::texture::Texture;

#[derive(Copy, Clone, Debug)]
pub struct Checker<'scene> {
    size: f32,
    even: &'scene dyn Texture,
    odd: &'scene dyn Texture,
}

impl<'scene> Checker<'scene> {
    pub fn  new(size: f32, even: &'scene dyn Texture, odd: &'scene dyn Texture) -> Self {
        Checker { size, even, odd }
    }
}

impl<'scene> Texture for Checker<'scene> {
    fn evaluate(&self, u: f32, v: f32) -> Vec3 {
        let x = (u / self.size) as usize;
        let y = (v / self.size) as usize;
        if (x & 1) ^ (y & 1) == 0 {
            self.even.evaluate(u, v)
        } else {
            self.odd.evaluate(u, v)
        }
    }
}
