use crate::{Ray, Hit, Vec3};

mod diffuse;
mod metal;

pub use diffuse::Diffuse;
pub use metal::Metal;

pub trait Material: std::fmt::Debug {
    fn scatter<'scene>(&self, ray: &Ray, hit: &'scene Hit, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

pub fn uniform_sphere() -> Vec3 {
    let ones = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let p = Vec3::new(
            rand::random(), 
            rand::random(), 
            rand::random(),
        ) * 2.0 - ones;
        if p.len_sq() < 1.0 { break p }
    }
}
