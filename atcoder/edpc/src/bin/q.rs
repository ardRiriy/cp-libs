use library::{
    data_structure::segtree::{monoids::RangeMaxMonoid, segment_tree::SegmentTree},
    utils::{input::Input, iterlibs::collect::CollectIter},
};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let h = ip.vector::<usize>(n);
    let h = h.iter().map(|i| *i - 1).collect_vec();
    let a = ip.vector::<u64>(n);

    let mut seg: SegmentTree<RangeMaxMonoid<u64>> = SegmentTree::new(n);
    for (i, hi) in h.iter().enumerate() {
        let prod = seg.get(..*hi);
        seg.set(*hi, prod + a[i]);
    }
    println!("{}", seg.get(..));
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = false;
    let mut ip = Input::new();

    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for _ in 0..t {
        solve(&mut ip);
    }
}
