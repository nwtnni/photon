use crate::arena;
use crate::bvh;
use crate::bxdf;
use crate::geom;
use crate::math;

#[derive(Clone, Debug)]
pub struct Mesh<'scene> {
    bxdf: &'scene dyn bxdf::BXDF,
    internal: bvh::Tree<'scene>,
}

impl<'scene> Mesh<'scene> {
    pub fn new(
        arena: &'scene arena::Arena,
        bxdf: &'scene dyn bxdf::BXDF,
        triangles: &[&'scene dyn geom::Surface<'scene>],
    ) -> Self {
        let internal = bvh::Tree::new(&triangles);
        use geom::Surface;
        println!("{:?}", internal.bound());
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
