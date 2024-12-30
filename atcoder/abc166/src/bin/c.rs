#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        h: [u64; n],
        e: [(Usize1, Usize1); m],
    }

    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });

    let ans = g.iter()
        .enumerate()
        .filter(|&(i, v)| {
            v.iter().all(|j| h[*j] < h[i])
        })
        .count();

    println!("{}", ans);
}

