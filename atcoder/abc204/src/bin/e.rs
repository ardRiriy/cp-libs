use std::{cmp::Reverse, collections::BinaryHeap};

use cps::consts::INF;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [(Usize1, Usize1, u64, u64); m],
    }
    let g = a.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v, c, d)| {
            g[u].push((v, c, d));
            g[v].push((u, c, d));
            g
        });

    let mut seen = vec![INF; n];
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0)));

    while let Some(Reverse((t, p))) = pq.pop() {
        if seen[p] != INF {
            continue;
        }
        if p == n-1 {
            println!("{}", t);
            return;
        }
        seen[p] = t;
        for &(ni, c, d) in g[p].iter() {
            if seen[ni] != INF {
                continue;
            }

            // f(t) = t + C + floor(D/(t+1)) を最小にするtは、round(√D)-1 となる
            
            let t_fmin = (d as f64).sqrt().round() as u64;
            let t = if t + 1 <= t_fmin { t_fmin - 1 } else { t }; 
            let min_t = t + c + d/(t+1);

            pq.push(Reverse((min_t, ni)));
        }
    }

    println!("-1");
}

