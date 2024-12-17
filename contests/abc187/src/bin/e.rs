#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn dfs1(g: &Vec<Vec<(usize, i64, i64)>>, p: usize, costs: &mut Vec<i64>, weights: &Vec<Vec<i64>>, seen: &mut Vec<bool>) {
    seen[p] = true;
    for &(c, _, mw) in &g[p] {
        if seen[c] {
            continue;
        }
        costs[0] += mw;
        dfs1(g, c, costs, weights, seen);
    }
}

fn dfs2(g: &Vec<Vec<(usize, i64, i64)>>, p: usize, ans: &mut Vec<i64>, seen: &mut Vec<bool>) {
    seen[p] = true;
    for &(ni, w, mw) in &g[p] {
        if seen[ni] {
            continue;
        }
        ans[ni] = ans[p] + w - mw;
        dfs2(g, ni, ans, seen);
    }
}

fn main() {
    input!{
        n: usize,
        e: [(Usize1, Usize1); n-1],
        q: usize,
        queries: [(Usize1, Usize1, i64); q],
    }


    let weights = queries.iter()
        .fold(vec![vec![0; 2]; n], |mut v, &(t, ei, x)| {
            v[ei][t] += x;
            v
        });
    
    let g = e.iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut v, (idx, &(a, b))| {
            v[a].push((b, weights[idx][1], weights[idx][0]));
            v[b].push((a, weights[idx][0], weights[idx][1]));
            v
        });

    let mut ans = vec![0; n];
    dfs1(&g, 0, &mut ans, &weights, &mut vec![false; n]);

    let mut seen = vec![false; n];
    seen[0] = true;
    dfs2(&g, 0, &mut ans, &mut seen);
    println!("{}", ans.iter().join("\n"));
}

