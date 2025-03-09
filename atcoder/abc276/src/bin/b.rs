use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }
    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u,v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });
    let ans = g.iter()
        .map(|v| (v.len(), v.iter().map(|i| *i+1).sorted_unstable().join(" ")))
        .map(|(l, s)| format!("{} {}", l, s))
        .join("\n");
    println!("{}", ans);
}

