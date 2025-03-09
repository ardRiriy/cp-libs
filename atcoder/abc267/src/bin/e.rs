use std::{cmp::Reverse, collections::BinaryHeap};

use cps::chlibs::ChLibs;
use itertools::cons_tuples;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [u64; n],
        e: [(Usize1, Usize1); m],
    }
    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });

    let mut costs = vec![0; n];
    for i in 0..n {
        for j in g[i].iter() {
            costs[*j] += a[i];
        }
    }

    let mut pq = BinaryHeap::from_iter(costs.iter().enumerate().map(|(i, c)| Reverse((*c, i))));
    let mut ans = 0;
    let mut seen = vec![false; n];
    while let Some(Reverse((c, i))) = pq.pop() {
        if seen[i] { continue; }
        seen[i] = true;
        ans.chmax(c);

        for ni in g[i].iter() {
            if seen[*ni] { continue; }
            costs[*ni] -= a[i];
            pq.push(Reverse((costs[*ni], *ni)));
        }
    }

    println!("{}", ans);
}

