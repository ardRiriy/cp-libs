
use ac_library::{Monoid, Segtree};
#[allow(unused_imports)]
use cps::debug::*;
use cps::veclibs::VecLibs;
use itertools::Itertools;
use proconio::input;

static INF: i64 = 1 << 60;

struct Max;
impl Monoid for Max {
    type S = (i64, usize);

    fn identity() -> Self::S {
        (-INF, 0)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.0 > b.0 {
            *a
        } else {
            *b
        }
    }
}


fn main() {
    input!{
        n: usize,
        m: usize,
        p: [i64; n],
        l: [i64; m],
        d: [i64; m],
    }

    let indicates = (0..m).sorted_unstable_by_key(|i| l[*i]).collect_vec();
    let mut sorted_l = vec![];
    let mut sorted_d = vec![];
    for &i in indicates.iter() {
        sorted_d.push(d[i]);
        sorted_l.push(l[i]);
    }

    let mut seg :Segtree<Max> = Segtree::new(m);
    for (i, &di) in sorted_d.iter().enumerate() {
        seg.set(i, (di, i));
    }

    let mut ans :i64 = p.iter().sum();

    for &pi in p.iter().sorted() {
        let i = sorted_l.lower_bound(pi+1); 
        let (v, idx) = seg.prod(..i);
        if v != -INF {
            ans -= v;
            seg.set(idx, (-INF, 0));
        }
    }
    println!("{ans}");
}

