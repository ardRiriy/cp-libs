use ac_library::Dsu;
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
        e: [(Usize1, Usize1); m],
        f: [(Usize1, Usize1); k],
    }

    let mut uf = Dsu::new(n);
    let mut v = vec![0; n];
    for &(a, b) in e.iter() {
        v[a] += 1;
        v[b] += 1;
        uf.merge(a, b);
    }
    
    let mut ans = (0..n).map(|i| uf.size(i) - v[i] - 1).collect_vec();
    for &(a, b) in f.iter() {
        if uf.same(a, b) {
            ans[a] -= 1;
            ans[b] -= 1;
        }
    }
    println!("{}", ans.iter().join(" "));
}
