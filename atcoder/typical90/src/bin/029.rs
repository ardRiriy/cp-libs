use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::{input, marker::Usize1};

struct Max;
impl Monoid for Max {
    type S = u64;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
}

struct Affine;
impl MapMonoid for Affine {
    type M = Max;
    type F = u64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        if *f == 0 { *x } else { *f }
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        if *f == 0 { *g } else { *f }
    }
}

fn main() {
    input! {
        w: usize,
        n: usize,
        block: [(Usize1, Usize1); n],
    }
    let mut segtree :LazySegtree<Affine> = LazySegtree::new(w);

    for (l, r) in block {
        let h = segtree.prod(l..=r);
        println!("{}", h + 1);
        segtree.apply_range(l..=r, h + 1);
    }
}
