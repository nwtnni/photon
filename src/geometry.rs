mod ray;
mod vec;

pub use ray::Ray;
pub use vec::Vec3;

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
