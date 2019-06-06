use crate::arena;
use crate::bvh;
use crate::geom;
use crate::material;
use crate::math;

#[derive(Clone, Debug)]
pub struct Mesh<'scene> {
    material: &'scene dyn material::Material<'scene>,
    internal: bvh::Linear<'scene>,
}

impl<'scene> Mesh<'scene> {
    pub fn new(
        arena: &'scene arena::Arena,
        material: &'scene dyn material::Material<'scene>,
        triangles: &[&'scene dyn geom::Surface<'scene>],
    ) -> Self {
        let internal = bvh::Linear::new(arena, &triangles);
        Mesh { material, internal }
    }
}

impl<'scene> geom::Surface<'scene> for Mesh<'scene> {
    fn bound(&self) -> geom::Bound {
        self.internal.bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        if self.internal.hit(ray, hit) {
            hit.m = Some(self.material);
            true
        } else {
            false
        }
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.internal.hit_any(ray)
    }
}
