mod axis;
mod ray;
mod vec;
mod mat;

pub use axis::Axis;
pub use ray::Ray;
pub use vec::Vec3;
pub use mat::Mat4;

use rand::random;

pub const PI: f32 = std::f32::consts::PI;
pub const FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2;
pub const TAU: f32 = PI * 2.0;
pub const EPSILON: f32 = 1e-5;

/// Floating point minimum
pub fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// Floating point maximum
pub fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

/// Restrict `a` to be between `lo` and `hi`, inclusive.
pub fn clamp(a: f32, lo: f32, hi: f32) -> f32 {
    max(lo, min(a, hi))
}

/// Generate a point in the unit sphere with uniform probability.
pub fn uniform_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(random(), random(), random()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if p.len_sq() < 1.0 { break p }
    }
}

/// Generate a point in the unit X-Y disk with uniform probability.
pub fn uniform_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random(), random(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
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
