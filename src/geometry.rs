mod bounds;

pub use self::bounds::*;

use noisy_float::prelude::*;

use crate::math::{Ray, Transform};

pub trait Shape {

    fn object_to_world(&self) -> &Transform;

    fn world_to_object(&self) -> &Transform;
    
    fn object_bound(&self) -> Bounds3f;

    fn world_bound(&self) -> Bounds3f {
        self.object_to_world() * self.object_bound()
    }

    fn intersect(&self, ray: &Ray) -> Option<N32>;
}
