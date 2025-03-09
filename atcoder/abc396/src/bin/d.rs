use std::u64;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1, u64); m],
    }
    
    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v, w)| {
        g[u].push((v, w));
        g[v].push((u, w));
        g
    });

    let ans = (1..n)
        .into_iter()
        .permutations(n-1)
        .filter_map(|v| {
            let mut prev = 0;
            let mut xorsum = 0;
            for pi in v.iter() {
                if let Some((_, w)) = g[prev].iter().find(|(t, _)| t == pi) {
                    xorsum ^= *w;
                    prev = *pi;
                } else {
                    return None;
                }
                if *pi==n-1 { break; }
            }
            Some(xorsum)
        })
        .min() 
        .unwrap();
    println!("{ans}");
}

