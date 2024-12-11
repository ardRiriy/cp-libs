use ac_library::Dsu;
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        e: [(Usize1, Usize1, u64); n-1],
    }

    let mut uf = Dsu::new(n);
    let ans = e.iter()
        .sorted_unstable_by_key(|&(_, _, w)| w)
        .fold(0, |mut sum, &(u, v, w)| {
            if uf.same(u, v) { return sum; }
            sum += uf.size(u) * uf.size(v) * w as usize;
            uf.merge(u, v);
            sum
        });
    println!("{ans}");
}

