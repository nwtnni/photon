use crate::geometry::{Ray, Vec3};
use crate::material::Material;

mod sphere;
mod list;

pub use list::List;
pub use sphere::Sphere;

#[derive(Copy, Clone, Debug, Default)]
pub struct Hit<'scene> {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub m: Option<&'scene dyn Material>,
}

pub trait Surface<'scene>: std::fmt::Debug + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut Hit<'scene>) -> bool;
}
