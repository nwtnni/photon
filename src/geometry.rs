mod ray;
mod vec;

pub use ray::Ray;
pub use vec::Vec3;

use rand::random;

/// Generate a point in the unit sphere with uniform probability.
pub fn uniform_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(random(), random(), random()) * 2.0 - vec::ONES_3D;
        if p.len_sq() < 1.0 { break p }
    }
}

/// Generate a point in the unit X-Y disk with uniform probability.
pub fn uniform_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random(), random(), 0.0) * 2.0 - vec::ONES_2D;
        if p.len_sq() < 1.0 { break p }
    }
}
