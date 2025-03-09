use std::collections::VecDeque;

#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
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

