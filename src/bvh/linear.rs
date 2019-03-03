use crate::bvh;
use crate::geometry::{Ray, Bound};
use crate::surface::{Hit, List, Surface};

#[derive(Clone, Debug)]
pub struct Linear<'scene>(Vec<Tree<'scene>>);

#[derive(Clone, Debug)]
enum Tree<'scene> {
    Leaf {
        bound: Bound,
        surface: &'scene Surface<'scene>,
    },
    List {
        bound: Bound,
        surfaces: List<'scene>,
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
        | Tree::List { bound, .. }
        | Tree::Node { bound, .. } => *bound,
        }
    }
}

impl<'scene> From<bvh::Tree<'scene>> for Linear<'scene> {
    fn from(tree: bvh::Tree<'scene>) -> Self {
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
        | bvh::Tree::List { bound, surfaces } => {
            nodes.push(Tree::List { bound, surfaces });
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

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit<'scene>) -> bool {

        let mut closest = t_max;
        let mut success = false;

        let dir = ray.d();
        let neg = [dir.x() < 0.0, dir.y() < 0.0, dir.z() < 0.0];

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

            if !node.bound().hit(ray, t_min, t_max, hit) { pop!(); continue }

            match node {
            | Tree::Leaf { surface, .. } => {
                if surface.hit(ray, t_min, closest, hit) {
                    success = true;
                    closest = hit.t;
                }
                pop!()
            }
            | Tree::List { surfaces, .. } => {
                if surfaces.hit(ray, t_min, closest, hit) {
                    success = true;
                    closest = hit.t;
                }
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
