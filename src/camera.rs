use noisy_float::prelude::*;

use crate::math::{Point2f, Point3f, Ray, Vec3f};

pub struct Camera<V: View> {
    view: V,
    width: N32,
    height: N32,
    position: Vec3f,
    u: Vec3f,
    v: Vec3f,
    w: Vec3f,
}

impl<V: View> Camera<V> {
    pub fn new(view: V, width: N32, height: N32, position: Vec3f, forward: Vec3f, up: Vec3f) -> Self {
        let w = -forward.normalize();
        let u = up.cross_v(&w).normalize();
        let v = w.cross_v(&u).normalize();
        Camera { view, width, height, position, u, v, w }
    }

    pub fn trace_through(&self, u: N32, v: N32) -> Ray {
        let x = u * self.width - (self.width / 2.0);
        let y = v * self.height - (self.height / 2.0);
        self.view.trace_through(self.position, self.u, self.v, self.w, x, y)
    }
}

pub trait View : Send + Sync {
    fn trace_through(&self, p: Vec3f, u: Vec3f, v: Vec3f, w: Vec3f, x: N32, y: N32) -> Ray;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Orthographic;

impl View for Orthographic {
    #[inline(always)]
    fn trace_through(&self, p: Vec3f, u: Vec3f, v: Vec3f, w: Vec3f, x: N32, y: N32) -> Ray {
        Ray::new(
            Point3f::from(p + (u * x) + v * y),
            -w,
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Perspective(N32);

impl Perspective {
    pub fn with_fov(fov: N32) -> Self {
        Perspective(fov.to_radians() / 2.0)
    }
}

impl View for Perspective {
    #[inline(always)]
    fn trace_through(&self, p: Vec3f, u: Vec3f, v: Vec3f, w: Vec3f, x: N32, y: N32) -> Ray {
        Ray::new(
            Point3f::from(p),
            -w + (u * self.0 * x) + (v * self.0 * y)
        )
    }
}
