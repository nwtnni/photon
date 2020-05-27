use crate::math;

mod lambertian;
mod specular;
mod fresnel;
mod glazed;
mod mirror;

pub use lambertian::Lambertian;
pub use specular::Specular;
pub use fresnel::Fresnel;
pub use glazed::Glazed;
pub use mirror::Mirror;

#[readonly::make]
#[derive(Copy, Clone, Debug, Default)]
pub struct Sample {
    /// Direction
    pub d: math::Vec3,

    /// Value
    pub v: math::Vec3,

    /// Probability
    pub p: f32,

    /// Whether this sample came from a delta distribution
    pub delta: bool,
}

pub trait BxDF: std::fmt::Debug + Send + Sync {
    fn eval(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> math::Vec3;
    fn sample(&self, d: &math::Vec3, n: &math::Vec3) -> Sample;
    fn pdf(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> f32;
}

impl<'a, B> BxDF for &'a B where B: BxDF + ?Sized {
    fn eval(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> math::Vec3 {
        (*self).eval(wi, wr, n)
    }

    fn sample(&self, d: &math::Vec3, n: &math::Vec3) -> Sample {
        (*self).sample(d, n)
    }

    fn pdf(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> f32 {
        (*self).pdf(wi, wr, n)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Any<'scene> {
    Lambertian(Lambertian),
    Specular(Specular),
    Glazed(Glazed<'scene>),
    Mirror(Mirror),
}

impl<'scene> BxDF for Any<'scene> {
    fn eval(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> math::Vec3 {
        match self {
            Any::Lambertian(bxdf) => bxdf.eval(wi, wr, n),
            Any::Specular(bxdf) => bxdf.eval(wi, wr, n),
            Any::Glazed(bxdf) => bxdf.eval(wi, wr, n),
            Any::Mirror(bxdf) => bxdf.eval(wi, wr, n),
        }
    }

    fn sample(&self, d: &math::Vec3, n: &math::Vec3) -> Sample {
        match self {
            Any::Lambertian(bxdf) => bxdf.sample(d, n),
            Any::Specular(bxdf) => bxdf.sample(d, n),
            Any::Glazed(bxdf) => bxdf.sample(d, n),
            Any::Mirror(bxdf) => bxdf.sample(d, n),
        }
    }

    fn pdf(&self, wi: &math::Vec3, wr: &math::Vec3, n: &math::Vec3) -> f32 {
        match self {
            Any::Lambertian(bxdf) => bxdf.pdf(wi, wr, n),
            Any::Specular(bxdf) => bxdf.pdf(wi, wr, n),
            Any::Glazed(bxdf) => bxdf.pdf(wi, wr, n),
            Any::Mirror(bxdf) => bxdf.pdf(wi, wr, n),
        }
    }
}
