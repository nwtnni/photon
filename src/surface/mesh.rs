use crate::arena::Arena;
use crate::bvh;
use crate::geom::{Bound, Ray};
use crate::material::Material;
use crate::surface::{Surface, Hit};

#[derive(Clone, Debug)]
pub struct Mesh<'scene> {
    material: &'scene dyn Material<'scene>,
    internal: bvh::Linear<'scene>,
}

impl<'scene> Mesh<'scene> {
    pub fn new(
        arena: &'scene Arena,
        material: &'scene dyn Material<'scene>,
        triangles: &[&'scene dyn Surface<'scene>],
    ) -> Self {
        let internal = bvh::Linear::new(arena, &triangles);
        Mesh { material, internal }
    }
}

impl<'scene> Surface<'scene> for Mesh<'scene> {
    fn bound(&self) -> Bound {
        self.internal.bound()
    }

    fn hit(&self, ray: &mut Ray, hit: &mut Hit<'scene>) -> bool {
        if self.internal.hit(ray, hit) {
            hit.m = Some(self.material);
            true
        } else {
            false
        }
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        self.internal.hit_any(ray)
    }
}
