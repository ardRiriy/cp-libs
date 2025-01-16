use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        e: [(Usize1, usize); q],
    }
    let g = e.iter().fold(vec![vec![]; n+1], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });
    let mut seen = vec![false; n+1];
    let mut que = VecDeque::new();
    que.push_back(0);
    seen[0]=true;
    while let Some(p) = que.pop_front() {
        for &ni in g[p].iter() {
            if !seen[ni] {
                seen[ni] = true;
                que.push_back(ni);
            }
        }
    }
    println!("{}", if seen[n]{"Yes"}else{"No"});
}