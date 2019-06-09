use crate::arena;
use crate::bvh;
use crate::bxdf;
use crate::geom;
use crate::math;

#[derive(Clone, Debug)]
pub struct Mesh<'scene> {
    bxdf: &'scene dyn bxdf::BXDF,
    internal: bvh::Linear<'scene>,
}

impl<'scene> Mesh<'scene> {
    pub fn new(
        bxdf: &'scene dyn bxdf::BXDF,
        triangles: &[&'scene dyn geom::Surface<'scene>],
    ) -> Self {
        let internal = bvh::Linear::new(&triangles);
        Mesh { bxdf, internal }
    }
}

impl<'scene> geom::Surface<'scene> for Mesh<'scene> {
    fn bound(&self) -> geom::Bound {
        self.internal.bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {
        if self.internal.hit(ray, hit) {
            hit.bxdf = Some(self.bxdf);
            true
        } else {
            false
        }
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        self.internal.hit_any(ray)
    }
}
