mod axis;
mod bound;
mod ray;
mod vec;

pub use axis::Axis;
pub use bound::Bound;
pub use ray::Ray;
pub use vec::Vec3;

use rand::random;

pub const PI: f32 = std::f32::consts::PI;
pub const TAU: f32 = PI * 2.0;

/// Floating point minimum
pub fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// Floating point maximum
pub fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

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

pub fn cosine_sphere() -> Vec3 {
    let u = random::<f32>();
    let v = random::<f32>();
    let r = u.sqrt();
    Vec3::new(
        r * (TAU * v).cos(),
        r * (TAU * v).sin(),
        (1.0 - u).sqrt(),
    )
}

pub fn basis(w: &Vec3) -> (Vec3, Vec3) {
    let x = w.x().abs();
    let y = w.y().abs();
    let z = w.z().abs();
    let v = if x <= y && x <= z {
        Vec3::new(1.0, 0.0, 0.0) 
    } else if y <= x && y <= z {
        Vec3::new(0.0, 1.0, 0.0) 
    } else {
        Vec3::new(0.0, 0.0, 1.0)
    };
    let u = v.cross(w).normalize();
    let v = w.cross(&u).normalize();
    (u, v)
}
