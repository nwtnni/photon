use noisy_float::prelude::*;

use crate::math::Num;

pub fn solve_quadratic(a: N32, b: N32, c: N32) -> Option<(N32, N32)> {
    let d_sq = b * b - n32(4.0) * a * c;

    if d_sq < n32(0.0) { return None }

    let d = d_sq.sqrt();

    let q = if b < 0.0 {
        n32(-0.5) * (b - d)
    } else {
        n32(-0.5) * (b + d)
    };

    match (q / a, c / a) {
    | (t0, t1) if t0 > t1 => Some((t1, t0)),
    | (t0, t1) => Some((t0, t1)),
    }
}

pub fn clamp<N: Num + num_traits::Float>(v: N, min: N, max: N) -> N {
    std::cmp::max(std::cmp::min(v, max), min)
}
