use std::{cmp::Reverse, collections::BinaryHeap};

use cps::consts::INF;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        h: [i64; n],
        e: [(Usize1, Usize1); m],
    }

    let mut g = vec![vec![]; n];
    for &(u, v) in e.iter() {
        if h[u] > h[v] {
            g[v].push((u, -(h[v]-h[u])));
            g[u].push((v, 0));
        } else {
            g[u].push((v, -(h[u]-h[v])));
            g[v].push((u, 0));
        }
    }

    let inf = -(INF as i64);
    let mut seen = vec![inf; n];
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0)));

    while let Some(Reverse((val, pos))) = pq.pop() {
        if seen[pos] != inf { continue; }
        seen[pos] = val;
        for &(ni, w) in g[pos].iter() {
            if seen[ni] == inf {
                pq.push(Reverse((val+w, ni)));
            }
        }
    }

    println!("{}", seen.iter().enumerate().map(|(i, &w)| h[0] - h[i] - w).max().unwrap());
}

