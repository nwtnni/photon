use crate::bvh;
use crate::math;
use crate::geom;

#[derive(Copy, Clone, Debug)]
pub struct Tree<'scene, S>(pub &'scene [Node<S>]);

impl<'scene, S> geom::Surface<'scene> for Tree<'scene, S> where S: geom::Surface<'scene> {
    fn bound(&self) -> geom::Box3 {
        self.0[0].bound()
    }

    fn hit(&self, ray: &mut math::Ray, hit: &mut geom::Hit<'scene>) -> bool {
        let mut this = 0;
        let mut visit = Vec::with_capacity(256);
        let mut success = false;

        macro_rules! push { ($i:expr) => {{
            visit.push($i);
        }}}

        macro_rules! pop { () => {{
            match visit.pop() {
            | None => break success,
            | Some(next) => this = next,
            }
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
            | Node::Leaf(leaf) => {
                success |= leaf.hit(ray, hit);
                pop!();
            }
            | Node::Node { children, .. } => {
                for child in children {
                    push!(*child as usize);
                }
                pop!();
            }
            }
        }
    }

    fn hit_any(&self, ray: &math::Ray) -> bool {
        let mut this = 0;
        let mut visit = Vec::with_capacity(256);

        macro_rules! push { ($i:expr) => {{
            visit.push($i);
        }}}

        macro_rules! pop { () => {{
            match visit.pop() {
            | None => return false,
            | Some(next) => this = next,
            }
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
            | Node::Leaf(leaf) => {
                if leaf.hit_any(ray) { return true }
                pop!();
            }
            | Node::Node { children, .. } => {
                for child in children {
                    push!(*child as usize);
                }
                pop!();
            }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Node<S> {
    Leaf(bvh::Leaf<S>),
    Node {
        bound: geom::Box3,
        children: [u32; 8],
    }
}

impl<'scene, S> Node<S> where S: geom::Surface<'scene> {
    pub fn bound(&self) -> geom::Box3 {
        use geom::Surface;
        match self {
        | Node::Leaf(leaf) => leaf.bound(),
        | Node::Node { bound, .. } => *bound,
        }
    }
}
