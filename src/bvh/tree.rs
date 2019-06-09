use std::collections::HashMap;

use crate::prelude::*;
use crate::math::{Axis, Ray, Vec3};
use crate::bvh::{Leaf, LEAF_SIZE};
use crate::geom;

const BUCKETS: usize = 12;

#[derive(Clone, Debug)]
pub enum Tree<'scene> {
    Leaf(Leaf<'scene>),
    Node {
        bound: geom::Bound,
        axis: Axis,
        l: Box<Tree<'scene>>,
        r: Box<Tree<'scene>>,
    },
}

impl<'scene> Tree<'scene> {
    pub fn new(surfaces: &[&'scene dyn Surface<'scene>]) -> Self {
        let lo = 0;
        let hi = surfaces.len();
        let mut info: Vec<Info> = surfaces.iter()
            .enumerate()
            .map(|(i, surface)| Info::new(i, surface.bound()))
            .collect();
        build(surfaces, &mut info, lo, hi)
    }

    pub fn len(&self) -> usize {
        match self {
        | Tree::Leaf(_) => 1,
        | Tree::Node { l, r, .. } => 1 + l.len() + r.len(),
        }
    }

    pub fn depth(&self) -> usize {
        match self {
        | Tree::Leaf(_) => 1,
        | Tree::Node { l, r, .. } => 1 + std::cmp::max(l.depth(), r.depth()),
        }
    }
}

impl<'scene> Surface<'scene> for Tree<'scene> {
    fn bound(&self) -> geom::Bound {
        match self {
        | Tree::Leaf(surfaces) => surfaces.bound(),
        | Tree::Node { bound, .. } => *bound,
        }
    }

    fn hit(&self, ray: &mut Ray, hit: &mut geom::Record<'scene>) -> bool {

        if !self.bound().hit_any(ray) {
            #[cfg(feature = "stats")]
            crate::stats::BVH_MISSES.inc();
            return false
        }

        #[cfg(feature = "stats")]
        crate::stats::BVH_HITS.inc();

        match self {
        | Tree::Leaf(surfaces) => {
            surfaces.hit(ray, hit)
        }
        | Tree::Node { bound, l, r, .. } => {
            let mut success = false;
            success |= l.hit(ray, hit);
            success |= r.hit(ray, hit);
            success
        },
        }
    }

    fn hit_any(&self, ray: &Ray) -> bool {

        if !self.bound().hit_any(ray) {
            #[cfg(feature = "stats")]
            crate::stats::BVH_MISSES.inc();
            return false
        }

        #[cfg(feature = "stats")]
        crate::stats::BVH_HITS.inc();

        match self {
        | Tree::Leaf(surfaces) => {
            surfaces.hit_any(ray)
        }
        | Tree::Node { bound, l, r, .. } => {
            l.hit_any(ray) || r.hit_any(ray)
        },
        }
    }
}

#[derive(Clone, Debug)]
struct Info {
    index: usize,
    bound: geom::Bound,
    centroid: Vec3,
}

impl Info {
    fn new(index: usize, bound: geom::Bound) -> Self {
        let centroid = bound.min() * 0.5 + bound.max() * 0.5;
        Info { index, bound, centroid }
    }
}

fn build<'scene>(
    surfaces: &[&'scene dyn Surface<'scene>],
    info: &mut [Info],
    lo: usize,
    hi: usize
) -> Tree<'scene> {

    let count = hi - lo;
    let bound = info[lo..hi].iter()
        .map(|info| info.bound)
        .fold(geom::Bound::smallest(), |a, b| a.union_b(&b));

    if count <= LEAF_SIZE {
        let mut leaf = Leaf::default();
        for (i, j) in (lo..hi).enumerate() {
            leaf.set(i, surfaces[info[j].index]);
        }
        return Tree::Leaf(leaf)
    }

    let centroid_bound = info[lo..hi].iter()
        .map(|info| info.centroid)
        .fold(geom::Bound::smallest(), |a, b| a.union_v(&b));

    let dim = centroid_bound.max_extent();

    let mid = if count <= 4 {

        info[lo..hi].sort_unstable_by(|a, b| {
            a.centroid[dim as usize]
                .partial_cmp(&b.centroid[dim as usize])
                .unwrap()
        });

        (lo + hi) / 2

    } else {

        let mut buckets = [(0, geom::Bound::smallest()); BUCKETS];
        let mut assignment: HashMap<usize, usize> = HashMap::default();
        for i in lo..hi {
            let o = centroid_bound.offset(&info[i].centroid)[dim as usize];
            let b = (BUCKETS as f32 * o) as usize;
            let b = std::cmp::min(b, BUCKETS - 1);
            buckets[b].0 += 1;
            buckets[b].1 = buckets[b].1.union_b(&info[i].bound);
            assignment.insert(info[i].index, b);
        }

        let mut cost = [0.0; BUCKETS - 1];
        for i in 0..BUCKETS - 1 {
            let mut left_bound = geom::Bound::smallest();
            let mut left_count = 0;
            for j in 0..=i {
                left_count += buckets[j].0;
                left_bound = left_bound.union_b(&buckets[j].1);
            }

            let mut right_bound = geom::Bound::smallest();
            let mut right_count = 0;
            for j in i + 1..BUCKETS {
                right_count += buckets[j].0;
                right_bound = right_bound.union_b(&buckets[j].1);
            }

            cost[i] = ( 
                left_count as f32 * left_bound.surface_area() +
                right_count as f32 * right_bound.surface_area()
            ) / bound.surface_area();
        }

        let mut min_cost = cost[0];
        let mut min_bucket = 0;
        for i in 1..BUCKETS - 1 {
            if cost[i] < min_cost {
                min_cost = cost[i];
                min_bucket = i;
            }
        }

        lo + partition::partition(
            &mut info[lo..hi],
            |info| assignment[&info.index] <= min_bucket
        ).0.len()
    };

    let l = build(surfaces, info, lo, mid);
    let r = build(surfaces, info, mid, hi);

    Tree::Node {
        axis: dim,
        bound: l.bound().union_b(&r.bound()),
        l: Box::new(l),
        r: Box::new(r),
    }
}
