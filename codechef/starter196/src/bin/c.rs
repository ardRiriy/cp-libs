use std::collections::{BTreeMap, BTreeSet};

use library::{data_structure::segment_tree::{Monoid, SegmentTree}, utils::input::Input};

struct Mono;
impl Monoid for Mono {
    type S = u64;

    fn op(a: Self::S, b: Self::S) -> Self::S {
        a+b
    }

    fn id() -> Self::S {
        0
    }
}


fn solve(input: &mut Input) {
    let (n, k) = input.pair::<usize, u64>();

    let mut v = Vec::from(&[input.vector::<u64>(n), input.vector::<u64>(n)]);
    let mut segs :Vec<SegmentTree<Mono>> = v.iter().map(|v| SegmentTree::from_vec(&v)).collect();

    

    let q = input.next::<usize>();
    for _ in 0..q {

    }
}

fn main() {
    let mut input = Input::new();
    let t = input.next::<usize>();
    for _ in 0..t {
        solve(&mut input);
    }
}
