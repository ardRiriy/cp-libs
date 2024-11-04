use std::collections::VecDeque;

use itertools::iproduct;
use proconio::{input, marker::Usize1};
fn solve() {
    input! {
        n: usize,
        m: usize,
        c: [u8; n],
        edges: [(Usize1, Usize1); m]
    }

    let g = edges.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });

    let mut visited = vec![vec![-1i64; n]; n];
    visited[0][n-1] = 0;
    let mut que = VecDeque::new();
    que.push_back((0, n-1));
    while let Some((tp, ap)) = que.pop_front() {
        for (&nt, &na) in iproduct!(g[tp].iter(), g[ap].iter()) {
            if visited[nt][na] != -1 || c[nt] == c[na] {
                continue;
            }
            visited[nt][na] = visited[tp][ap] + 1;
            que.push_back((nt, na));
        }
    }
    println!("{}", visited[n-1][0]);
}

fn main() {
    input!{
        mut t: usize,
    }

    while t > 0 {
        solve();
        t -= 1;
    }
}
