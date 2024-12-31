use std::ops::Index;

use ac_library::{LazySegtree, MapMonoid, Monoid, ModInt998244353 as Mint};
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

struct Sum;
impl Monoid for Sum {
    type S = Mint;

    fn identity() -> Self::S {
        Mint::new(0)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a+b
    }
}

struct Affine;
impl MapMonoid for Affine {
    type M = Sum;
    type F = Mint;

    fn identity_map() -> Self::F {
        Mint::new(1)
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f*x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f*g
    }
}

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }

    let indicates = (0..n).sorted_by_key(|i| a[*i]).collect_vec(); // a[i] ga nanbanme
    let indicates = indicates.iter().enumerate().fold(vec![0; n], |mut v, (i, j)| {v[*j] = i; v});

    let mut segtree :LazySegtree<Affine> = LazySegtree::new(n);
    let mut ans = Mint::new(0);
    for (i, _) in a.iter().enumerate() {
        ans += segtree.prod(..indicates[i]);
        segtree.apply_range(0..n, Mint::new(2));
        segtree.set(indicates[i], Mint::new(1));
    }
    println!("{}", ans);
}

