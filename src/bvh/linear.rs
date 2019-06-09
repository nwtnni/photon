use crate::prelude::*;
use crate::arena;
use crate::bvh;
use crate::geom;
use crate::math;

#[derive(Clone, Debug)]
pub struct Linear<'scene>(&'scene [Tree<'scene>]);

impl<'scene> Linear<'scene> {
    pub fn new(arena: &'scene arena::Arena, surfaces: &[&'scene dyn Surface<'scene>]) -> Self {
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
    Leaf(bvh::Leaf<'scene>),
    Node {
        axis: math::Axis,
        bound: geom::Bound,
        offset: u32,
    },
}

impl<'scene> Tree<'scene> {
    fn bound(&self) -> geom::Bound {
        match self {
        | Tree::Leaf(leaf) => leaf.bound(),
        | Tree::Node { bound, .. } => *bound,
        }
    }
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
    fn bound(&self) -> geom::Bound {
        match self.0[0] {
        | Tree::Leaf(surface) => surface.bound(),
        | Tree::Node { bound, .. } => bound,
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
