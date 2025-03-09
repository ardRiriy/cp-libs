use std::collections::VecDeque;

use cps::potentiality_unionfind::{PotentialMergeOp, PotentialityUnionfind};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

#[derive(Default)]
struct XorSum;
impl PotentialMergeOp<i64> for XorSum {
    fn identity() -> i64 {
        0
    }

    fn merge(a: i64, b: i64) -> i64 {
        a^b
    }

    fn invert(a: i64) -> i64 {
        a
    }
}

fn solve2() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1, i64); m],
    }

    let mut ans = vec![0; n];

    for i in 0..32 {
        let mut uf = PotentialityUnionfind::new(n, Some(XorSum));
        for &(u, v, w) in &e {
            if let Err(_) = uf.merge(u, v, (w>>i)&1) {
                println!("-1");
                return;
            }
        }
        let mut cnt = vec![0; n];
        for i in 0..n {
            let leader = uf.leader(i);
            cnt[leader] += uf.diff(i, leader).unwrap();
        }
        for j in 0..n {
            let leader = uf.leader(j);
            let one = cnt[leader];
            let zero = uf.size(leader) as i64 - one;
            if one > zero && uf.diff(j, leader).unwrap() == 0 {
                ans[j] |= 1 << i;
            } else if one <= zero && uf.diff(j, leader).unwrap() == 1 {
                ans[j] |= 1 << i;
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}

#[allow(dead_code)]
fn solve1() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1, u64); m],
    }

    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v, w)| {
        g[u].push((v, w));
        g[v].push((u, w));
        g
    });
    let mut ans = vec![0; n];

    static INF: u64 = 100;
    for i in 0..32 {
        // ibit目を決める
        let mut seen = vec![INF; n];
        for j in 0..n {
            if seen[j] != INF { continue; }
            let mut reached = vec![j];

            seen[j] = 0;
            let mut que = VecDeque::new();
            que.push_back(j);

            let mut sum = 0;
            while let Some(p) = que.pop_front() {
                for &(ni, w) in &g[p] {
                    if seen[ni] == INF {
                        if (w>>i) & 1 == 0 {
                            seen[ni] = seen[p];
                        } else {
                            seen[ni] = seen[p] ^ 1;
                        }
                        sum += seen[ni];
                        reached.push(ni);
                        que.push_back(ni);
                    } else {
                        if (w>>i) & 1 == 0 {
                            if seen[ni] != seen[p] {
                                println!("-1");
                                return;
                            }
                        } else {
                            if seen[ni] == seen[p] {
                                println!("-1");
                                return;
                            }
                        }
                    }
                }
            }
            for &p in reached.iter() {
                if sum > reached.len() as u64 - sum {
                    seen[p] ^= 1;
                }
                ans[p] |= seen[p]<<i; 
            }
        }
    }
    println!("{}", ans.iter().join(" "));
}

fn main() {
    solve2();
}

