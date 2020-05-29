use std::collections::HashMap;
use std::iter;

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

#[derive(Copy, Clone, Debug, Default)]
struct Bucket {
    /// Bounding box
    bound: geom::Box3,

    /// Surfaces in bucket
    count: usize,
}

impl<'b> iter::FromIterator<&'b Bucket>  for Bucket {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = &'b Bucket> {
        iter.into_iter()
            .fold(Bucket::default(), |a, b| Bucket {
                bound: a.bound | b.bound,
                count: a.count + b.count,
            })
    }
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

        build(&mut bvh, surfaces, &mut info);

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
) where S: geom::Surface<'scene> + Copy {
    let bound = info
        .iter()
        .map(|Info { bound, .. }| bound)
        .collect::<geom::Box3>();

    if info.len() == 1 {
        let mut leaf = bvh::Leaf::default();
        leaf.set(0, surfaces[info[0].index]);
        bvh.push(bvh::Node::Leaf(leaf));
        return
    }

    let centroid_bound = info
        .iter()
        .map(|Info { center, .. }| center)
        .collect::<geom::Box3>();

    let dim = centroid_bound.max_extent();

    let mid = if info.len() <= 4 {

        info.sort_unstable_by(|a, b| {
            a.center
                .get(dim as usize)
                .partial_cmp(&b.center.get(dim as usize))
                .expect("[INTERNAL ERROR]: floating point comparison failed")
        });

        info.len() / 2

    } else {

        let mut buckets = [Bucket::default(); BUCKETS];
        let mut assignment: HashMap<usize, usize> = HashMap::default();

        for Info { bound, center, index } in info.iter() {
            let o = centroid_bound.offset(center).get(dim as usize);
            let b = (BUCKETS as f32 * o) as usize;
            let b = std::cmp::min(b, BUCKETS - 1);
            buckets[b].count += 1;
            buckets[b].bound |= bound;
            assignment.insert(*index, b);
        }

        let mut costs = [0.0; BUCKETS - 1];

        for (index, cost) in costs.iter_mut().enumerate() {
            let left = buckets
                .iter()
                .take(index + 1)
                .collect::<Bucket>();

            let right = buckets
                .iter()
                .skip(index + 1)
                .collect::<Bucket>();

            *cost = 1.0 + ( 
                left.count as f32 * left.bound.surface_area() +
                right.count as f32 * right.bound.surface_area()
            ) / bound.surface_area();
        }

        let (min_bucket, min_cost) = costs
            .iter()
            .copied()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.partial_cmp(b).expect("[INTERNAL ERROR]: floating point comparison failed")
            })
            .expect("[INTERNAL ERROR]: no minimum BVH cost");

        if info.len() > bvh::LEAF_SIZE || min_cost < info.len() as f32 {

            let (half, _) = partition::partition(info, |Info { index, .. }| assignment[index] <= min_bucket);
            half.len()

        } else {

            let mut leaf = bvh::Leaf::default();
            for (leaf_index, Info { index: surface_index, .. }) in info.iter().enumerate() {
                leaf.set(leaf_index, surfaces[*surface_index]);
            }
            bvh.push(bvh::Node::Leaf(leaf));
            return;

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

    build(bvh, surfaces, &mut info[..mid]);

    let child = bvh.len();

    build(bvh, surfaces, &mut info[mid..]);

    bvh[index] = bvh::Node::Node {
        axis: dim,
        bound: bvh[index + 1].bound() | bvh[child].bound(),
        child: child as u32, 
    };
}
