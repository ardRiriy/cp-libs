use std::cmp::Reverse;

use ac_library::{LazySegtree, MapMonoid, Monoid};
use itertools::Itertools;
use proconio::input;

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
    
    fn identity_element() -> <Self::M as ac_library::Monoid>::S {
        Self::M::identity()
    }
    
    fn binary_operation(
        a: &<Self::M as ac_library::Monoid>::S,
        b: &<Self::M as ac_library::Monoid>::S,
    ) -> <Self::M as ac_library::Monoid>::S {
        Self::M::binary_operation(a, b)
    }
}

fn main() {
    input!{
        h: usize,
        w: usize,
        n: usize,
        bars: [(usize, usize, usize); n],
    }
    let indicates = (0..n).sorted_unstable_by_key(|i| Reverse(bars[*i])).collect_vec();
    let mut ans = vec![0; n];
    
    let mut segtree :LazySegtree<Affine> = LazySegtree::new(w+1);
    for i in indicates {
        let (_, c, l) = bars[i];
        let x = segtree.prod(c..c+l);
        ans[i] = h as u64-x;
        segtree.apply_range(c..c+l, x+1);
    }

    println!("{}", ans.iter().join("\n"));
}

