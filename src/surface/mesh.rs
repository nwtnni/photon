use crate::arena::Arena;
use crate::bvh;
use crate::geometry::{Bound, Ray};
use crate::material::Material;
use crate::surface::{Surface, Hit};

#[derive(Clone, Debug)]
pub struct Mesh<'scene> {
    material: &'scene dyn Material<'scene>,
    internal: bvh::Linear<'scene>,
    // internal: bvh::Tree<'scene>,
    // internal: crate::surface::List<'scene>,
}

impl<'scene> Mesh<'scene> {
    pub fn new(
        arena: &'scene Arena,
        material: &'scene dyn Material<'scene>,
        triangles: &[&'scene dyn Surface<'scene>],
        t_min: f32,
        t_max: f32,
    ) -> Self {
        let internal = bvh::Linear::new(arena, &triangles, t_min, t_max);
        // let internal = bvh::Tree::new(&triangles, t_min, t_max);
        // let mut internal = crate::surface::List::with_capacity(triangles.len());
        // for triangle in triangles { internal.push(*triangle); }
        Mesh { material, internal }
    }
}

impl<'scene> Surface<'scene> for Mesh<'scene> {
    fn bound(&self, t_min: f32, t_max: f32) -> Bound {
        self.internal.bound(t_min, t_max)
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
