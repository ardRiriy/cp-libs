use std::collections::VecDeque;

use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

static INF: usize = 1<<60;

fn main() {
    input!{
        n: usize,
        m: usize,
        k: i64,
        a: [Usize1; m],
        e: [(Usize1, Usize1); n-1],
    }

    let g = e.iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut g, (i, &(u, v))| {
            g[u].push((v, i));
            g[v].push((u, i));
            g
        });

    let mut w = vec![0; n-1];
    for (&cur, &np) in a.iter().tuple_windows() {
        if cur == np {
            continue;
        }

        let mut seen = vec![(INF, INF); n];
        seen[cur] = (!0, !0);
        let mut que = VecDeque::new();
        que.push_back(cur);
        while let Some(p) = que.pop_front() {
            if p == np {
                break;
            }
            for &(ni, i) in g[p].iter() {
                if seen[ni].0 == INF {
                    seen[ni] = (p, i);
                    que.push_back(ni);
                }
            }
        }

        {
            let mut p = np;
            while seen[p].0 != !0 {
                w[seen[p].1] += 1;
                p = seen[p].0;
            }
        }
    }

    let sum = w.iter().sum::<u64>();
    let x = sum as i64 + k;
    if x%2==1 || x < 0 {
        println!("0");
        return;
    }

    let x = x as usize;
    let mut dp = vec![vec![Mint::new(0); x+1]; 2];
    let mut cur = 0;
    dp[cur][0] = Mint::new(1);
    for i in 0..n-1 {
        let nxt = 1-cur;
        for j in 0..x+1 {
            if w[i] as usize + j < x+1 {
                dp[nxt][w[i] as usize + j] = dp[nxt][w[i] as usize + j] + dp[cur][j];
            }
            dp[nxt][j] = dp[nxt][j] + dp[cur][j];
        }
        dp[cur].fill(Mint::new(0));
        cur = nxt;
    }
    println!("{}", dp[cur][x/2]);
}
