use crate::prelude::*;
use crate::arena;
use crate::bvh;
use crate::geom;
use crate::math;

#[derive(Copy, Clone, Debug)]
pub struct Linear<'scene, S>(&'scene [Tree<S>]);

impl<'scene, S> Linear<'scene, S> where S: Surface<'scene> + Copy {
    pub fn new(arena: &'scene arena::Arena, surfaces: &[S]) -> Self {
        unsafe {
            let tree = bvh::Tree::new(surfaces);
            let mut nodes = arena.alloc_slice_mut(tree.len());
            tree.flatten(&mut nodes, &mut 0);
            Linear(nodes)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Tree<S> {
    Leaf(bvh::Leaf<S>),
    Node {
        axis: math::Axis,
        bound: geom::Box3,
        offset: u32,
    },
}

impl<'scene, S> Tree<S> where S: Surface<'scene> {
    fn bound(&self) -> geom::Box3 {
        match self {
        | Tree::Leaf(leaf) => leaf.bound(),
        | Tree::Node { bound, .. } => *bound,
        }
    }
}

impl<'scene, S> bvh::Tree<S> {
    fn flatten(self, nodes: &'scene mut [Tree<S>], index: &mut usize) {
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

impl<'scene, S> Surface<'scene> for Linear<'scene, S> where S: Surface<'scene> {
    fn bound(&self) -> geom::Box3 {
        match &self.0[0] {
        | Tree::Leaf(surface) => surface.bound(),
        | Tree::Node { bound, .. } => *bound,
        }
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Record<'scene>) -> bool {

        let mut next = 0;
        let mut this = 0;
        let mut visit = [0; 32];
        let mut success = false;

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

            if !self.0[this].bound().hit_any(ray) {
                #[cfg(feature = "stats")] 
                crate::stats::BVH_MISSES.inc();
                pop!();
                continue
            }

            #[cfg(feature = "stats")] 
            crate::stats::BVH_HITS.inc(); 

            match &self.0[this] {
            | Tree::Leaf(surfaces) => {
                success |= surfaces.hit(ray, hit);
                pop!();
            }
            | Tree::Node { offset, axis, .. } => {
                if ray.sign[*axis as usize] == 1 {
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

    fn hit_any(&self, ray: &math::Ray) -> bool {
        let mut next = 0;
        let mut this = 0;
        let mut visit = [0; 32];

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

            if !self.0[this].bound().hit_any(ray) {
                #[cfg(feature = "stats")]
                crate::stats::BVH_MISSES.inc();
                pop!();
                continue
            }

            #[cfg(feature = "stats")]
            crate::stats::BVH_HITS.inc();

            match &self.0[this] {
            | Tree::Leaf(surfaces) => {
                if surfaces.hit_any(ray) { return true }
                pop!();
            }
            | Tree::Node { axis, offset, .. } => {
                if ray.sign[*axis as usize] == 1 {
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
