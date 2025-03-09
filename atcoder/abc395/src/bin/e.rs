use std::{cmp::Reverse, collections::BinaryHeap};

use cps::consts::INF;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        x: u64,
        e: [(Usize1, Usize1); m],
    }

    let g = e.iter()
        .fold(vec![vec![vec![]; n]; 2], |mut g, &(u, v)| {
            g[0][u].push(v);
            g[1][v].push(u);
            g
        });

    let mut seen = vec![vec![INF; n]; 2];
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, 0, 0)));
    while let Some(Reverse((c, i, p))) = pq.pop() {
        if seen[i][p] != INF { continue; }
        seen[i][p] = c;

        for &ni in g[i][p].iter() {
            if seen[i][ni] == INF {
                pq.push(Reverse((c+1, i, ni)));
            }
        }

        if seen[1-i][p] == INF {
            pq.push(Reverse((c+x, 1-i, p)));
        }
    }
    println!("{}", seen[0][n-1].min(seen[1][n-1]));
}

