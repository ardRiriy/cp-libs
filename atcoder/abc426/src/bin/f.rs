use library::{
    data_structure::segtree::{
        lazy_segment_tree::LazySegmentTree,
        monoids::{MapMonoid, Monoid},
    },
    utils::input::Input,
};

struct S;
impl Monoid for S {
    type S = i64;

    fn op(a: Self::S, b: Self::S) -> Self::S {
        a.min(b)
    }

    fn id() -> Self::S {
        0
    }
}
struct F;
impl MapMonoid for F {
    type M = S;
    type F = i64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}

fn solve(ip: &mut Input) {
    let n = ip.next();
    let a = ip.vector(n);
    let mut seg: LazySegmentTree<F> = LazySegmentTree::new(n);
    for i in 0..n {
        seg.set(i, a[i]);
    }

    for _ in 0..ip.next() {
        let (l, r) = ip.pair::<usize>();
        let k = ip.next::<i64>();
    }
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
