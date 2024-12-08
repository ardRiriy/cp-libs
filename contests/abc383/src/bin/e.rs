#[allow(unused_imports)]
use cps::debug::*;
use cps::unionfind::UnionFind;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
        e: [(Usize1, Usize1, u64); m],
        a: [Usize1; k],
        b: [Usize1; k],
    }
    let mut ans = 0;
    let f = |x: &(u64, u64), y: &(u64, u64)| {
        let a = x.0 + y.0;
        let b = x.1 + y.1;
        if a > b {
            (a - b, 0)
        } else {
            (0, b - a)
        }
    };

    let mut uf = UnionFind::new(n, f);

    let va = a.iter().fold(vec![0; n], |mut v, &ai| { v[ai]+=1; v });
    let vb = b.iter().fold(vec![0; n], |mut v, &ai| { v[ai]+=1; v });
    for i in 0..n {
        uf.insert_data(i, (va[i], vb[i]));
    }

    for &(u, v, w) in e.iter().sorted_unstable_by_key(|v| v.2) {
        if uf.same(u, v) {
            continue;
        }
        let (pa, pb) = uf.get_data(u).copied().unwrap();
        let (qa, qb) = uf.get_data(v).copied().unwrap();
        let val = (pa+qa).min(pb+qb);
        ans += val * w;
        uf.merge(u, v);
    }
    println!("{}", ans);
}

