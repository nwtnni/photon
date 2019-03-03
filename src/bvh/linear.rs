use crate::bvh;
use crate::geometry::{Ray, Bound};
use crate::surface::{Hit, Surface};

#[derive(Clone, Debug)]
pub struct Linear<'scene>(Vec<Tree<'scene>>);

#[derive(Clone, Debug)]
enum Tree<'scene> {
    Leaf {
        bound: Bound,
        surface: &'scene Surface<'scene>,
    },
    Node {
        bound: Bound,
        axis: u8,
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

impl<'scene> From<&'scene bvh::Tree<'scene>> for Linear<'scene> {
    fn from(tree: &'scene bvh::Tree<'scene>) -> Self {
        let mut nodes = Vec::with_capacity(tree.len());
        tree.flatten(&mut nodes);
        Linear(nodes)
    }
}

impl<'scene> bvh::Tree<'scene> {
    fn flatten(self, nodes: &mut Vec<Tree<'scene>>) {
        match self {
        | bvh::Tree::Leaf { bound, surface } => {
            nodes.push(Tree::Leaf { bound, surface });
        }
        | bvh::Tree::Node { bound, axis, l, r } => {
            let offset = l.len() + 1;
            nodes.push(Tree::Node { bound, axis, offset });
            l.flatten(nodes);
            r.flatten(nodes)
        }
        }
    }
}

impl<'scene> Surface<'scene> for Linear<'scene> {
    fn bound(&self, _: f32, _: f32) -> Bound {
        unreachable!()
    }

    fn hit(&self, ray: &mut Ray, hit: &mut Hit<'scene>) -> bool {

        let mut success = false;

        let neg = [ray.d.x() < 0.0, ray.d.y() < 0.0, ray.d.z() < 0.0];

        let mut next = 0;
        let mut this = 0;
        let mut visit = [0; 32];

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
            | Tree::Node { axis, offset, .. } => {
                if neg[*axis as usize] {
                    push!(this + 1); 
                    this += *offset;
                } else {
                    push!(this + *offset);
                    this += 1;
                }
            }
            }
        }
    }
}
