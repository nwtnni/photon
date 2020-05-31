#![allow(unused)]

use std::ops::BitXor;

const MIN: f32 = 2.3283064365386963e-10;
const MAX: f32 = 1.0f32 - f32::EPSILON;

const COLS: usize = 52;
const DIMS: usize = 1024;

static MATRICES: [u32; DIMS * COLS] = include!("../data/sobol.txt");

/// Sobol sampler
#[derive(Clone, Debug)]
pub struct Sobol;

impl Sobol {
    fn sample(
        mut input: u64,
        dimension: usize,
    ) -> f32 {
        let mut output = 0;
        let mut column = dimension * COLS;
        while input != 0 {
            if input & 1 > 0 {
                output ^= MATRICES[column];
            }
            input >>= 1;
            column += 1;
        }
        input as f32 * MIN
    }
}
