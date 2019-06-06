use crate::prelude::*;
use crate::bvh;
use crate::arena::Arena;
use crate::geom::{Axis, Ray, Bound};
use crate::bvh::Leaf;
use crate::surface;

#[derive(Clone, Debug)]
pub struct Linear<'scene>(&'scene [Tree<'scene>]);

impl<'scene> Linear<'scene> {
    pub fn new(arena: &'scene Arena, surfaces: &[&'scene dyn Surface<'scene>]) -> Self {
        unsafe {
            let tree = bvh::Tree::new(surfaces);
            let mut nodes = arena.alloc_slice_uninitialized(tree.len());
            let mut index = 0;
            tree.flatten(&mut nodes, &mut index);
            Linear(nodes)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Tree<'scene> {
    Leaf(Leaf<'scene>),
    Node {
        axis: Axis,
        bound: Bound,
        offset: u32,
    },
}

impl<'scene> bvh::Tree<'scene> {
    fn flatten(self, nodes: &mut [Tree<'scene>], index: &mut usize) {
        match self {
        | bvh::Tree::Leaf(surfaces) => {
            nodes[*index] = Tree::Leaf(surfaces);
            *index += 1;
        }
        | bvh::Tree::Node { bound, axis, l, r } => {
            nodes[*index] = Tree::Node { axis, bound, offset: l.len() as u32 + 1, };
            *index += 1;
            l.flatten(nodes, index);
            r.flatten(nodes, index);
        }
        }
    }
}

impl<'scene> Surface<'scene> for Linear<'scene> {
    fn bound(&self) -> Bound {
        match self.0[0] {
        | Tree::Leaf(surface) => surface.bound(),
        | Tree::Node { bound, .. } => bound,
        }
    }

    fn hit(&self, ray: &mut Ray, hit: &mut surface::Record<'scene>) -> bool {

        let mut next = 0;
        let mut this = 0;
        let mut visit = [0; 32];
        let mut success = false;
        let inv = [1.0 / ray.d.x(), 1.0 / ray.d.y(), 1.0 / ray.d.z()];
        let neg = [ray.d.x() < 0.0, ray.d.y() < 0.0, ray.d.z() < 0.0];

        macro_rules! push { ($i:expr) => {{
            visit[next] = $i; 
            next += 1;
        }}}

        macro_rules! pop { () => {{
            if next == 0 { break success }
            next -= 1;
            this = visit[next];
        }}}

        loop {
            match &self.0[this] {
            | Tree::Leaf(surfaces) => {
                if surfaces.hit(ray, hit) { success = true; }
                pop!();
            }
            | Tree::Node { bound, offset, axis, .. } => {
                if !bound.hit_inv(ray, &inv) {
                    pop!();
                } else if neg[*axis as usize] {
                    push!(this + 1);
                    this += *offset as usize;
                } else {
                    push!(this + *offset as usize);
                    this += 1;
                }
            }
            }
        }
    }

    fn hit_any(&self, ray: &Ray) -> bool {
        let mut next = 0;
        let mut this = 0;
        let mut visit = [0; 32];
        let inv = [1.0 / ray.d.x(), 1.0 / ray.d.y(), 1.0 / ray.d.z()];
        let neg = [ray.d.x() < 0.0, ray.d.y() < 0.0, ray.d.z() < 0.0];

        macro_rules! push { ($i:expr) => {{
            visit[next] = $i; 
            next += 1;
        }}}

        macro_rules! pop { () => {{
            if next == 0 { return false }
            next -= 1;
            this = visit[next];
        }}}

        loop {
            match &self.0[this] {
            | Tree::Leaf(surfaces) => {
                if surfaces.hit_any(ray) { return true }
                pop!();
            }
            | Tree::Node { axis, bound, offset, .. } => {
                if !bound.hit_inv(ray, &inv) {
                    pop!();
                } else if neg[*axis as usize] {
                    push!(this + 1);
                    this += *offset as usize;
                } else {
                    push!(this + *offset as usize);
                    this += 1;
                }
            }
            }
        }

    }
}
