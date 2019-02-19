use crate::{Ray, Vec3};

mod sphere;

pub use sphere::Sphere;

#[derive(Copy, Clone, Debug)]
pub struct Hit {
    t: f32,
    p: Vec3,
    n: Vec3,
}

pub trait Surface {
    fn hit(ray: &Ray, t_min: f32, t_max: f32, record: &mut Hit) -> bool;
}
