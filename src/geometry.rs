use crate::{Ray, Vec3};

mod sphere;
mod list;

pub use list::List;
pub use sphere::Sphere;

#[derive(Copy, Clone, Debug, Default)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
}

pub trait Surface: std::fmt::Debug {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut Hit) -> bool;
}
