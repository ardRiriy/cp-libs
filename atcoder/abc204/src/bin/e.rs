use std::{cmp::Reverse, collections::BinaryHeap};

use cps::chlibs::ChLibs;
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

            let mut l = t;
            let mut r = INF/10; 

            while r - l > 2 {
                let m1 = l + (r - l) / 3;
                let m2 = r - (r - l) / 3;

                if d / (t + 1 + m1) + m1 > d / (t + 1 + m2) + m2 {
                    l = m1;
                } else {
                    r = m2;
                }
            }
            let mut min_t = INF;
            for i in l..=r {
                min_t.chmin(t+c+d/(t+1+i)+i);
            }
            pq.push(Reverse((min_t, ni)));
        }
    }

    println!("-1");
}

