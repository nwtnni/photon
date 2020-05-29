use std::arch::x86_64;
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
    if info.len() < 4 {
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
            bounds: bvh::tree::Box3SoA::default(),
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

    let (bound, bounds) = unsafe {
        let child_0 = bvh[children[0] as usize].bound();
        let child_1 = bvh[children[1] as usize].bound();
        let child_2 = bvh[children[2] as usize].bound();
        let child_3 = bvh[children[3] as usize].bound();
        let child_4 = bvh[children[4] as usize].bound();
        let child_5 = bvh[children[5] as usize].bound();
        let child_6 = bvh[children[6] as usize].bound();
        let child_7 = bvh[children[7] as usize].bound();

        let bound = child_0 | child_1 | child_2 | child_3 | child_4 | child_5 | child_6 | child_7;

        let bounds = bvh::tree::Box3SoA {
            min_x: x86_64::_mm256_set_ps(
                child_7.min.x(),
                child_6.min.x(),
                child_5.min.x(),
                child_4.min.x(),
                child_3.min.x(),
                child_2.min.x(),
                child_1.min.x(),
                child_0.min.x(),
            ),
            min_y: x86_64::_mm256_set_ps(
                child_7.min.y(),
                child_6.min.y(),
                child_5.min.y(),
                child_4.min.y(),
                child_3.min.y(),
                child_2.min.y(),
                child_1.min.y(),
                child_0.min.y(),
            ),
            min_z: x86_64::_mm256_set_ps(
                child_7.min.z(),
                child_6.min.z(),
                child_5.min.z(),
                child_4.min.z(),
                child_3.min.z(),
                child_2.min.z(),
                child_1.min.z(),
                child_0.min.z(),
            ),

            max_x: x86_64::_mm256_set_ps(
                child_7.max.x(),
                child_6.max.x(),
                child_5.max.x(),
                child_4.max.x(),
                child_3.max.x(),
                child_2.max.x(),
                child_1.max.x(),
                child_0.max.x(),
            ),
            max_y: x86_64::_mm256_set_ps(
                child_7.max.y(),
                child_6.max.y(),
                child_5.max.y(),
                child_4.max.y(),
                child_3.max.y(),
                child_2.max.y(),
                child_1.max.y(),
                child_0.max.y(),
            ),
            max_z: x86_64::_mm256_set_ps(
                child_7.max.z(),
                child_6.max.z(),
                child_5.max.z(),
                child_4.max.z(),
                child_3.max.z(),
                child_2.max.z(),
                child_1.max.z(),
                child_0.max.z(),
            ),
        };

        (bound, bounds)
    };

    bvh[parent] = bvh::Node::Node {
        bound,
        bounds,
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
