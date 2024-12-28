#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1); n - 1],
    }

    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });
    let mut ans = vec![];
    dfs(&g, &mut ans, &mut vec![false; n], 0);
    println!("{}", ans.iter().map(|&x| x + 1).join(" "));
}

fn dfs(g: &Vec<Vec<usize>>, ans: &mut Vec<usize>, seen: &mut Vec<bool>, p: usize) {
    seen[p] = true;
    ans.push(p);
    for &ni in g[p].iter().sorted() {
        if seen[ni] {
            continue;
        }
        dfs(g, ans, seen, ni);
        ans.push(p);
    }
}
