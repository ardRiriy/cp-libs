use std::collections::VecDeque;

use cps::consts::INF;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        k: usize,
        edges: [(Usize1, Usize1); n-1],
        v: [Usize1; k]
    }
    let g = edges.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });

    let mut in_size = g
        .iter()
        .map(|v| v.len())
        .collect_vec();

    for &u in &v {
        in_size[u] = INF as usize;
    }

    let mut que = VecDeque::new();
    for (idx, &size) in in_size.iter().enumerate() {
        if size == 1 {
            que.push_back(idx);
        }
    }

    let mut ans = n;
    while let Some(u) = que.pop_front() {
        in_size[u] = INF as usize;
        ans -= 1;
        for &v in &g[u] {
            in_size[v] -= 1;
            if in_size[v] == 1 {
                que.push_back(v);
            }
        }
    }

    println!("{}", ans);
}
