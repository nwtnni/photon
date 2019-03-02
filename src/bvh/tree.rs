use std::collections::HashMap;

use crate::geometry::{Bound, Ray, Vec3};
use crate::surface::{Hit, List, Surface};

const BUCKETS: usize = 12;

#[derive(Clone, Debug)]
pub enum Tree<'scene> {
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
        l: Box<Tree<'scene>>,
        r: Box<Tree<'scene>>,
    },
}

impl<'scene> Tree<'scene> {
    pub fn new(
        surfaces: &'scene [&'scene dyn Surface<'scene>],
        maximum: usize,
        ts: f32,
        tf: f32,
    ) -> Self {
        let lo = 0;
        let hi = surfaces.len();
        let mut info: Vec<Info> = surfaces.iter()
            .enumerate()
            .map(|(i, surface)| Info::new(i, surface.bound(ts, tf)))
            .collect();
        build(surfaces, &mut info, lo, hi, maximum, ts, tf)
    }

    pub fn len(&self) -> usize {
        match self {
        | Tree::Leaf { .. } => 1,
        | Tree::List { .. } => 1,
        | Tree::Node { l, r, .. } => 1 + l.len() + r.len(),
        }
    }
}

impl<'scene> Surface<'scene> for Tree<'scene> {
    fn bound(&self, _: f32, _: f32) -> Bound {
        match self {
        | Tree::Leaf { bound, .. }
        | Tree::List { bound, .. }
        | Tree::Node { bound, .. } => *bound,
        }
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit<'scene>) -> bool {
        if !self.bound(0.0, 0.0).hit(ray, t_min, t_max, hit) { return false }
        match self {
        | Tree::Leaf { surface, .. } => surface.hit(ray, t_min, t_max, hit),
        | Tree::List { surfaces, .. } => surfaces.hit(ray, t_min, t_max, hit),
        | Tree::Node { l, r, .. } => {
            let mut record = Hit::default();
            let mut closest = t_max;
            let mut success = false;
            if l.hit(ray, t_min, closest, &mut record) {
                success = true;
                closest = record.t;
                *hit = record;
            }
            if r.hit(ray, t_min, closest, &mut record) {
                success = true;
                *hit = record;
            }
            success
        },
        }
    }
}

#[derive(Clone, Debug)]
struct Info {
    index: usize,
    bound: Bound,
    centroid: Vec3,
}

impl Info {
    fn new(index: usize, bound: Bound) -> Self {
        let centroid = bound.min() * 0.5 + bound.max() * 0.5;
        Info { index, bound, centroid }
    }
}

fn build<'scene>(
    surfaces: &'scene [&'scene dyn Surface<'scene>],
    info: &mut [Info],
    lo: usize,
    hi: usize,
    maximum: usize,
    ts: f32,
    tf: f32,
) -> Tree<'scene> {

    let count = hi - lo;
    let bound = info[lo..hi].iter()
        .map(|info| info.bound)
        .fold(Bound::smallest(), |a, b| a.union_b(&b));

    macro_rules! leaf {
        () => {{
            let mut list = List::with_capacity(count);
            for i in lo..hi { list.push(surfaces[info[i].index]); }
            return Tree::List { bound, surfaces: list }
        }}
    };

    if count == 1 {
        return Tree::Leaf {
            bound,
            surface: surfaces[info[lo].index],
        }
    }

    let centroid_bound = info[lo..hi].iter()
        .map(|info| info.centroid)
        .fold(Bound::smallest(), |a, b| a.union_v(&b));

    let dim = centroid_bound.max_extent();
    if centroid_bound.max()[dim as usize]
    == centroid_bound.min()[dim as usize] { leaf!() }

    let mid = if count <= 4 {

        info[lo..hi].sort_unstable_by(|a, b| {
            a.centroid[dim as usize]
                .partial_cmp(&b.centroid[dim as usize])
                .unwrap()
        });

        (lo + hi) / 2

    } else {

        let mut buckets = [(0, Bound::smallest()); BUCKETS];
        let mut assignment: HashMap<usize, usize> = HashMap::default();
        for i in lo..hi {
            let o = centroid_bound.offset(&info[i].centroid)[dim as usize];
            let b = (BUCKETS as f32 * o).round() as usize;
            let b = std::cmp::min(b, BUCKETS - 1);
            buckets[b].0 += 1;
            buckets[b].1 = buckets[b].1.union_b(&info[i].bound);
            assignment.insert(info[i].index, b);
        }

        let mut cost = [0.0; BUCKETS - 1];
        for i in 0..BUCKETS - 1 {
            let mut left_bound = Bound::smallest();
            let mut left_count = 0;
            for j in 0..=i {
                left_count += buckets[j].0;
                left_bound = left_bound.union_b(&buckets[j].1);
            }

            let mut right_bound = Bound::smallest();
            let mut right_count = 0;
            for j in i + 1..BUCKETS {
                right_count += buckets[j].0;
                right_bound = right_bound.union_b(&buckets[j].1);
            }

            cost[i] = 0.125 + (
                left_count as f32 * left_bound.surface_area() +
                right_count as f32 * right_bound.surface_area()
            ) / bound.surface_area();
        }

        let leaf_cost = count as f32;
        let mut min_cost = cost[0];
        let mut min_bucket = 0;
        for i in 1..BUCKETS - 1 {
            if cost[i] < min_cost {
                min_cost = cost[i];
                min_bucket = i;
            }
        }

        if count <= maximum && min_cost >= leaf_cost { leaf!() }

        lo + partition::partition(
            &mut info[lo..hi],
            |info| assignment[&info.index] <= min_bucket
        ).0.len()
    };

    let l = build(surfaces, info, lo, mid, maximum, ts, tf);
    let r = build(surfaces, info, mid, hi, maximum, ts, tf);
    Tree::Node {
        axis: dim,
        bound: l.bound(ts, tf).union_b(&r.bound(ts, tf)),
        l: Box::new(l),
        r: Box::new(r),
    }
}
