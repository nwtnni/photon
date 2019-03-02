use crate::arena::Arena;
use crate::geometry::{Bound, Ray, Vec3};
use crate::surface::{Hit, List, Surface};

const BUCKETS: usize = 12;

#[derive(Clone, Debug)]
pub enum Tree<'arena, 'scene> {
    Leaf {
        bound: Bound,
        surfaces: List<'scene>,
    },

    Node {
        bound: Bound,
        axis: u8,
        children: [&'arena Tree<'arena, 'scene>; 2],
    },
}

impl<'arena, 'scene> Tree<'arena, 'scene> {
    pub fn new(
        arena: &'arena Arena<Tree<'arena, 'scene>>,
        surfaces: &'scene [&'scene dyn Surface<'scene>],
        maximum: usize,
        ts: f32,
        tf: f32,
    ) -> &'arena Self {
        let lo = 0;
        let hi = surfaces.len();
        let mut info: Vec<Info> = surfaces.iter()
            .enumerate()
            .map(|(i, surface)| Info::new(i, surface.bound(ts, tf)))
            .collect();
        build(surfaces, arena, &mut info, lo, hi, maximum, ts, tf)
    }
}

impl<'arena, 'scene> Surface<'scene> for &Tree<'arena, 'scene> {
    fn bound(&self, _: f32, _: f32) -> Bound {
        match self {
        | Tree::Leaf { bound, .. }
        | Tree::Node { bound, .. } => *bound,
        }
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit<'scene>) -> bool {
        if !self.bound(0.0, 0.0).hit(ray, t_min, t_max, hit) { return false }
        match self {
        | Tree::Leaf { surfaces, .. } => surfaces.hit(ray, t_min, t_max, hit),
        | Tree::Node { children, .. } => {
            let mut record = Hit::default();
            let mut closest = t_max;
            let mut success = false;
            if children[0].hit(ray, t_min, closest, &mut record) {
                success = true;
                closest = record.t;
                *hit = record;
            }
            if children[1].hit(ray, t_min, closest, &mut record) {
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

fn build<'arena, 'scene>(
    surfaces: &'scene [&'scene dyn Surface<'scene>],
    arena: &'arena Arena<Tree<'arena, 'scene>>,
    info: &mut [Info],
    lo: usize,
    hi: usize,
    maximum: usize,
    ts: f32,
    tf: f32,
) -> &'arena Tree<'arena, 'scene> {

    let count = hi - lo;
    let bound = info[lo..hi].iter()
        .map(|info| info.bound)
        .fold(Bound::smallest(), |a, b| a.union_b(&b));

    macro_rules! return_leaf {
        () => {{
            let mut list = List::with_capacity(count);
            for i in lo..hi { list.push(surfaces[info[i].index]); }
            return arena.alloc(Tree::Leaf { bound, surfaces: list })
        }}
    };

    if count == 1 { return_leaf!() }

    let centroid_bound = info[lo..hi].iter()
        .map(|info| info.centroid)
        .fold(Bound::smallest(), |a, b| a.union_v(&b));

    let dim = centroid_bound.max_extent();
    if centroid_bound.max()[dim as usize]
    == centroid_bound.min()[dim as usize] { return_leaf!() }

    let mid = if count <= 4 {

        info[lo..hi].sort_unstable_by(|a, b| {
            a.centroid[dim as usize]
                .partial_cmp(&b.centroid[dim as usize])
                .unwrap()
        });

        (lo + hi) / 2

    } else {

        let mut buckets = [(0, Bound::smallest()); BUCKETS];
        let mut assignment = vec![0; count];
        for i in lo..hi {
            let o = centroid_bound.offset(&info[i].centroid)[dim as usize];
            let b = (BUCKETS as f32 * o).floor() as usize;
            let b = std::cmp::min(b, BUCKETS - 1);
            buckets[b].0 += 1;
            buckets[b].1 = buckets[b].1.union_b(&info[i].bound);
            assignment[info[i].index] = b;
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

        if count <= maximum && min_cost >= leaf_cost { return_leaf!() }

        lo + partition::partition(
            &mut info[lo..hi],
            |info| assignment[info.index] <= min_bucket
        ).0.len()
    };

    let l = build(surfaces, arena, info, lo, mid, maximum, ts, tf);
    let r = build(surfaces, arena, info, mid, hi, maximum, ts, tf);
    arena.alloc(Tree::Node {
        axis: dim,
        bound: l.bound(ts, tf).union_b(&r.bound(ts, tf)),
        children: [l, r],
    })
}
