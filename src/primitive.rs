use crate::material::Material;
use crate::geometry::Shape;
use crate::math::Transform;

pub struct Primitive<'scene> {
    material: &'scene Material,
    shape: &'scene Shape, 
    transform: Transform,
}
