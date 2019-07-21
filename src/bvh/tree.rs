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
            | Node::Leaf(leaf) => {
                success |= leaf.hit(ray, hit);
                pop!();
            }
            | Node::Node { child, axis, .. } => {
                if ray.sign[*axis as usize] == 1 {
                    push!(this + 1);
                    this = *child as usize;
                } else {
                    push!(*child as usize);
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
            | Node::Leaf(leaf) => {
                if leaf.hit_any(ray) { return true }
                pop!();
            }
            | Node::Node { axis, child, .. } => {
                if ray.sign[*axis as usize] == 1 {
                    push!(this + 1);
                    this = *child as usize;
                } else {
                    push!(*child as usize);
                    this += 1;
                }
            }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Node<S> {
    Leaf(bvh::Leaf<S>),
    Node {
        axis: math::Axis,
        bound: geom::Box3,
        child: u32,
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
