use crate::geometry::{Bound, Ray, Vec3};
use crate::material::Material;
use crate::surface::{Hit, Surface};

#[derive(Copy, Clone, Debug)]
pub struct Tri<'scene> {
    material: &'scene Material,
    vertices: [&'scene Vec3; 3],
    normals: [&'scene Vec3; 3],
}

impl<'scene> Tri<'scene> {
    pub fn new(
        material: &'scene Material,
        vertices: [&'scene Vec3; 3],
        normals: [&'scene Vec3; 3],
    ) -> Self {
        Tri { material, vertices, normals }
    }
}

impl<'scene> Surface<'scene> for Tri<'scene> {
    fn bound(&self, _: f32, _: f32) -> Bound {
        Bound::new(*self.vertices[0], *self.vertices[1])
            .union_v(self.vertices[2])
    }

    fn hit(&self, ray: &mut Ray, hit: &mut Hit<'scene>) -> bool {
        const EPSILON: f32 = 0.0000001;

        let edge_a = self.vertices[1] - self.vertices[0];
        let edge_b = self.vertices[2] - self.vertices[0];
        let h = ray.d.cross(&edge_b);
        let det = edge_a.dot(&h);

        if det > -EPSILON && det < EPSILON { return false }

        let inv = 1.0 / det;
        let s = ray.o - self.vertices[0];
        let u = inv * s.dot(&h);
        if u < 0.0 || u > 1.0 { return false }

        let q = s.cross(&edge_a);
        let v = inv * ray.d.dot(&q);
        if v < 0.0 || u + v > 1.0 { return false }

        let t = inv * edge_b.dot(&q);
        if t < ray.min || t > ray.max { return false }
        let w = 1.0 - u - v;

        ray.max = t;
        hit.t = t;
        hit.m = Some(self.material);
        hit.p =
            self.vertices[0] * w +
            self.vertices[1] * u +
            self.vertices[2] * v;
        hit.n = (
            self.normals[0] * w +
            self.normals[1] * u +
            self.normals[2] * v
        ).normalize();
        true
    }
}
