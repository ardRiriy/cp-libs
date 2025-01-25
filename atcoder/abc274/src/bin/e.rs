#[allow(unused_imports)]
use cps::debug::*;
use num_traits::Pow;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        mut pos: [(f64, f64); n+m],
    }

    pos.insert(n, (0., 0.));
    pos.insert(0, (0., 0.));

    let dist = |from: usize, to: usize| -> f64 {
        ((pos[from].0 - pos[to].0).powf(2.) + (pos[from].1 - pos[to].1).powf(2.)).sqrt()
    };

    // 0, n+1: 原点 
    // i=1, ..., n 街i-1 (0-indexed)
    // i=n+2, ..., n+m+1 宝箱i-n-2 (0-indexed)
    static INF :f64 = 1e18;
    let mut dp = vec![vec![INF; n+m+2]; 1<<(n+m+2)];
    dp[1][0] = 0.;

    for i in 1..1<<(n+m+2) {
        let mut boost = 0;
        for j in n+2..n+m+2 {
            boost += (i>>j)&1;
        }

        for j in 0..n+m+2 {
            if dp[i][j] == INF { continue; }

            for k in 0..n+m+2 {
                if (i>>k)&1 == 1 { continue; } // 到達済み

                let ni = i | 1<<k;
                let d = dist(j, k);

                dp[ni][k] = dp[ni][k].min(dp[i][j] + d / 2.pow(boost) as f64);
            }
        }
    }
    let mut ans = INF;
    'l: for i in 1..1<<(n+m+2) {
        for j in 0..n+2 {
            if (i >> j) & 1 == 0 { continue 'l; }
        }

        ans = ans.min(dp[i][n+1]);
    }
    println!("{}", ans);
}

