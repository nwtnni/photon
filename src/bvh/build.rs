use std::cmp;
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
) where S: geom::Surface<'scene> + Copy,
{
    if info.len() < bvh::LEAF_SIZE {
        let mut leaf = bvh::Leaf::default();
        for (leaf_index, Info { index: surface_index, .. }) in info.iter().enumerate() {
            leaf.set(leaf_index, surfaces[*surface_index]);
        }
        bvh.push(bvh::Node::Leaf(leaf));
        return;
    }

    let subset_bound = info
        .iter()
        .map(|Info { bound, .. }| bound)
        .collect::<geom::Box3>();

    let center_bound = info
        .iter()
        .map(|Info { center, .. }| center)
        .collect::<geom::Box3>();

    let [a, b, c] = center_bound.max_extent();

    let mid = partition(info, center_bound, subset_bound, a);

    let (lo_mid, hi_mid) = (
        partition(&mut info[..mid], center_bound, subset_bound, b),
        mid + partition(&mut info[mid..], center_bound, subset_bound, b),
    );

    let (lo_lo_mid, hi_lo_mid, lo_hi_mid, hi_hi_mid) = (
        partition(&mut info[..lo_mid], center_bound, subset_bound, c),
        lo_mid + partition(&mut info[lo_mid..mid], center_bound, subset_bound, c),
        mid + partition(&mut info[mid..hi_mid], center_bound, subset_bound, c),
        hi_mid + partition(&mut info[hi_mid..], center_bound, subset_bound, c),
    );

    let mut children = [0u32; 8];

    let parent = bvh.len();

    // Placeholder node
    bvh.push(
        bvh::Node::Node {
            bound: geom::Box3::default(),
            children,
        }
    );

    for (index, range) in [
            0..lo_lo_mid,
            lo_lo_mid..lo_mid,
            lo_mid..hi_lo_mid,
            hi_lo_mid..mid,
            mid..lo_hi_mid,
            lo_hi_mid..hi_mid,
            hi_mid..hi_hi_mid,
            hi_hi_mid..info.len(),
        ]
        .iter()
        .cloned()
        .enumerate()
    {
        children[index] = bvh.len() as u32;
        build(bvh, surfaces, &mut info[range]);
    }

    let bound = children
        .iter()
        .copied()
        .map(|child| bvh[child as usize].bound())
        .collect::<geom::Box3>();

    bvh[parent] = bvh::Node::Node {
        bound,
        children,
    };
}

fn partition(
    info: &mut [Info],
    center_bound: geom::Box3,
    subset_bound: geom::Box3,
    dim: math::Axis,
) -> usize {
    let mut buckets = [Bucket::default(); BUCKETS];
    let mut assignment: HashMap<usize, usize> = HashMap::default();

    for Info { bound, center, index } in info.iter() {
        let o = center_bound.offset(center).get(dim as usize);
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
        ) / subset_bound.surface_area();
    }

    let (min, _) = costs
        .iter()
        .copied()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            if a.is_nan() {
                cmp::Ordering::Greater
            } else if b.is_nan() {
                cmp::Ordering::Less
            } else {
                a.partial_cmp(b).unwrap()
            }
        })
        .expect("[INTERNAL ERROR]: no minimum BVH cost");

    let (half, _) = partition::partition(info, |Info { index, .. }| assignment[index] <= min);

    half.len()
}
