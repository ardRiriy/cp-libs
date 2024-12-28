use std::{collections::VecDeque, usize};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

const INF: u64 = 1 << 62;

fn main() {
    input!{
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, i64); m],
    }

    let g = edges.iter().fold(vec![vec![]; n], |mut g, &(u, v, w)| {
        g[u].push((v, w));
        g[v].push((u, -w));
        g
    });

    let mut ans = vec![INF as i64; n];
    for i in 0..n {
        if ans[i] != INF as i64{
            continue;
        }

        let mut que = VecDeque::new();
        que.push_back(i);
        ans[i] = 0;
        while let Some(p) = que.pop_front() {
            for &(u, w) in g[p].iter() {
                if ans[u] != INF as i64{
                    continue;
                }
                // ans[u] - ans[p] = w
                ans[u] = w + ans[p];
                que.push_back(u);
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}
