use crate::arena;
use crate::light;
use crate::surface;

pub struct Scene<'scene> {
    lights: Vec<&'scene dyn light::Light>,
    surfaces: Vec<&'scene dyn surface::Surface<'scene>>,
}

impl<'scene> Scene<'scene> {
    pub fn new(arena: &arena::Arena) -> Self {
        unimplemented!()
    }
}
