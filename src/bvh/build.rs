use std::collections::HashMap;

use crate::arena;
use crate::bvh;
use crate::math;
use crate::geom;

const BUCKETS: usize = 12;

#[derive(Debug)]
struct Info {
    /// Index in original array
    index: usize,

    /// Bounding box
    bound: geom::Box3,

    /// Centroid point
    center: math::Vec3,
}

impl<'scene, S> bvh::Tree<'scene, S> where S: geom::Surface<'scene> + Copy, {
    pub fn new(arena: &'scene arena::Arena, surfaces: &[S]) -> Self {

        let mut bvh = Vec::new();
        let mut info = surfaces.iter()
            .enumerate()
            .map(|(i, s)| {
                let b = s.bound();
                let c = b.min * 0.5 + b.max * 0.5;
                Info { index: i, bound: b, center: c }
            })
            .collect::<Vec<_>>();

        let lo = 0;
        let hi = surfaces.len();

        build(&mut bvh, surfaces, &mut info, lo, hi);

        unsafe {
            let arr = arena.alloc_slice_mut(bvh.len());
            arr.copy_from_slice(&bvh);
            bvh::Tree(arr)
        }
    }
}

fn build<'scene, S>(
    bvh: &mut Vec<bvh::Node<S>>, 
    surfaces: &[S],
    info: &mut [Info],
    lo: usize,
    hi: usize
)
    where S: geom::Surface<'scene> + Copy,
{
    let count = hi - lo;
    let bound = info[lo..hi].iter()
        .map(|info| info.bound)
        .fold(geom::Box3::smallest(), |a, b| a.union_b(&b));

    if count == 1 {
        let mut leaf = bvh::Leaf::default();
        leaf.set(0, surfaces[info[lo].index]);
        bvh.push(bvh::Node::Leaf(leaf));
        return
    }

    let centroid_bound = info[lo..hi].iter()
        .map(|info| info.center)
        .fold(geom::Box3::smallest(), |a, b| a.union_v(&b));

    let dim = centroid_bound.max_extent();

    let mid = if count <= 4 {

        info[lo..hi].sort_unstable_by(|a, b| {
            a.center.get(dim as usize)
                .partial_cmp(&b.center.get(dim as usize))
                .unwrap()
        });

        (lo + hi) / 2

    } else {

        let mut buckets = [(0, geom::Box3::smallest()); BUCKETS];
        let mut assignment: HashMap<usize, usize> = HashMap::default();
        for i in lo..hi {
            let o = centroid_bound.offset(&info[i].center).get(dim as usize);
            let b = (BUCKETS as f32 * o) as usize;
            let b = std::cmp::min(b, BUCKETS - 1);
            buckets[b].0 += 1;
            buckets[b].1 = buckets[b].1.union_b(&info[i].bound);
            assignment.insert(info[i].index, b);
        }

        let mut cost = [0.0; BUCKETS - 1];
        for i in 0..BUCKETS - 1 {
            let mut left_bound = geom::Box3::smallest();
            let mut left_count = 0;
            for j in 0..=i {
                left_count += buckets[j].0;
                left_bound = left_bound.union_b(&buckets[j].1);
            }

            let mut right_bound = geom::Box3::smallest();
            let mut right_count = 0;
            for j in i + 1..BUCKETS {
                right_count += buckets[j].0;
                right_bound = right_bound.union_b(&buckets[j].1);
            }

            cost[i] = 1.0 + ( 
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

        if count > bvh::LEAF_SIZE || min_cost < count as f32 {

            lo + partition::partition(
                &mut info[lo..hi],
                |info| assignment[&info.index] <= min_bucket
            ).0.len()
        
        } else {
            let mut leaf = bvh::Leaf::default();
            for (i, j) in (lo..hi).enumerate() {
                leaf.set(i, surfaces[info[j].index]);
            }
            bvh.push(bvh::Node::Leaf(leaf));
            return
        }
    };

    let index = bvh.len();

    // Place dummy node
    bvh.push(
        bvh::Node::Node {
            axis: dim,
            bound: geom::Box3::default(),
            child: 0,
        }
    );

    build(bvh, surfaces, info, lo, mid);

    let child = bvh.len();

    build(bvh, surfaces, info, mid, hi);

    bvh[index] = bvh::Node::Node {
        axis: dim,
        bound: bvh[index + 1].bound().union_b(&bvh[child].bound()),
        child: child as u32, 
    };
}
