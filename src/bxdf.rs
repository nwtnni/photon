use crate::geometry::Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Sample {
    n: Vec3,
    wi: Vec3,
    wo: Vec3,
    discrete: bool,
}

pub trait BXDF {
    fn eval(wi: Vec3, wo: Vec3, n: Vec3, out: &mut Vec3);
    fn sample(wi: Vec3, sample: &mut Sample, out: &mut Vec3) -> f32;
    fn pdf(wi: Vec3, wo: Vec3, n: Vec3) -> f32;
}
