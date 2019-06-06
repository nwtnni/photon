use crate::math::{Ray, Vec3};
use crate::integrator;
use crate::scene;
use crate::surface;

#[derive(Copy, Clone, Debug)]
pub struct Normal;

impl<'scene> integrator::Integrator<'scene> for Normal {
    fn shade(_: &scene::Scene<'scene>, _: &Ray, hit: &surface::Record<'scene>, _: usize) -> Vec3 {
        Vec3::new(
            (hit.n.x() + 1.0) / 2.0,
            (hit.n.y() + 1.0) / 2.0,
            (hit.n.z() + 1.0) / 2.0,
        )
    }
}
