use std::cmp::Reverse;

use cps::chlibs::ChLibs;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn dfs(g: &Vec<Vec<usize>>, p: usize, dp: &mut Vec<Vec<(usize, u64)>>, seen: &mut Vec<bool>) -> u64 {
    seen[p] = true;
    for &ni in g[p].iter() {
        if seen[ni] { continue; }
        let res = dfs(g, ni, dp, seen);
        dp[p].push((ni, res));
    }

    while dp[p].len() < 6 {
        dp[p].push((!0, 1));
    }

    dp[p].iter().sorted_unstable_by_key(|xi| Reverse(xi.1)).take(3).map(|xi| xi.1).sum::<u64>()+1
} 

fn dfs2(p: usize, dp: &mut Vec<Vec<(usize, u64)>>, seen: &mut Vec<bool>) -> u64 {
    seen[p] = true;
    let mut res = dp[p].iter().sorted_unstable_by_key(|xi| Reverse(xi.1)).take(4).map(|xi| xi.1).sum::<u64>()+1;

    for i in 0..dp[p].len() {
        let (ni, _) = dp[p][i];
        if ni == !0 || seen[ni] { continue; }
        let n_cost = dp[p].iter()
            .filter(|xi| xi.0 != ni)
            .sorted_unstable_by_key(|xi| Reverse(xi.1))
            .map(|xi| xi.1)
            .take(3)
            .sum::<u64>();

        dp[ni].push((!0, n_cost));
        let res2 = dfs2(ni, dp, seen);
        res = res.max(res2);
    }
    res
}

fn main() {
    input!{
        n: usize,
        e: [(Usize1, Usize1); n-1],
    }
    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });

    let n_g = e.iter()
        .fold(vec![vec![]; n], |mut n_g, &(u, v)| {
            if g[u].len() >= 4 && g[v].len() >= 4 {
                n_g[u].push(v);
                n_g[v].push(u);
            }
            n_g
        });
    let mut checked = vec![false; n];
    let mut checked2 = vec![false; n];
    let mut dp = vec![vec![]; n];
    let mut ans = -1i64;

    for i in 0..n {
        if g[i].len() < 4 || checked[i] { continue; }
        dfs(&n_g, i, &mut dp, &mut checked);
        let size = dfs2(i, &mut dp, &mut checked2);
        ans.chmax(size as i64); 
    }

    println!("{}", ans);
}

