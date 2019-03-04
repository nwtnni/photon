use crate::bvh;
use crate::arena::Arena;
use crate::geometry::{Ray, Bound};
use crate::surface::{Hit, Surface};

#[derive(Clone, Debug)]
pub struct Linear<'scene>(&'scene [Tree<'scene>]);

impl<'scene> Linear<'scene> {
    pub fn new(
        arena: &'scene Arena,
        surfaces: &[&'scene dyn Surface<'scene>],
        t_min: f32,
        t_max: f32,
    ) -> Self {
        unsafe {
            let tree = bvh::Tree::new(surfaces, t_min, t_max);
            let mut nodes = arena.alloc_slice_uninitialized(tree.len());
            let mut index = 0;
            tree.flatten(&mut nodes, &mut index);
            Linear(nodes)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Tree<'scene> {
    Leaf {
        bound: Bound,
        surface: &'scene dyn Surface<'scene>,
    },
    Node {
        bound: Bound,
        offset: usize,
    },
}

impl<'scene> Tree<'scene> {
    fn bound(&self) -> Bound {
        match self {
        | Tree::Leaf { bound, .. }
        | Tree::Node { bound, .. } => *bound,
        }
    }
}

impl<'scene> bvh::Tree<'scene> {
    fn flatten(self, nodes: &mut [Tree<'scene>], index: &mut usize) {
        match self {
        | bvh::Tree::Leaf { bound, surface } => {
            nodes[*index] = Tree::Leaf { bound, surface };
            *index += 1;
        }
        | bvh::Tree::Node { bound, l, r } => {
            nodes[*index] = Tree::Node { bound, offset: l.len() + 1, };
            *index += 1;
            l.flatten(nodes, index);
            r.flatten(nodes, index);
        }
        }
    }
}

impl<'scene> Surface<'scene> for Linear<'scene> {
    fn bound(&self, _: f32, _: f32) -> Bound {
        match self.0[0] {
        | Tree::Leaf { bound, .. }
        | Tree::Node { bound, .. } => bound,
        }
    }

    fn hit(&self, ray: &mut Ray, hit: &mut Hit<'scene>) -> bool {

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
            let node = &self.0[this];

            if !node.bound().hit(ray, hit) { pop!(); continue }

            match node {
            | Tree::Leaf { surface, .. } => {
                if surface.hit(ray, hit) { success = true; }
                pop!()
            }
            | Tree::Node { offset, .. } => {
                push!(this + *offset);
                this += 1;
            }
            }
        }
    }
}
