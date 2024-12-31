use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        v: [(i64, i64, i64); n],
    }

    let inf = 1<<60;
    let mut dp = vec![vec![inf; n]; 1<<n];
    dp[1][0] = 0;

    let cost = |f: usize, t: usize| -> i64 {
        (v[t].0-v[f].0).abs() + (v[t].1-v[f].1).abs() + (v[t].2-v[f].2).max(0)
    };

    for i in 1..1<<n {
        for k in 0..n {
            if (i >> k) & 1 == 0 {
                continue;
            }
            for l in k+1..n {
                if (i >> l) & 1 == 0 {
                    continue;
                }

                let kl_cost = dp[i][k] + cost(k, l);
                let lk_cost = dp[i][l] + cost(l, k);

                dp[i][l].chmin(kl_cost);
                dp[i][k].chmin(lk_cost);
            }
        }

        for j in 0..n {
            if (i >> j) & 1 == 0 {
                continue;
            }
            for k in 0..n {
                if (i >> k) & 1 == 1 {
                    continue;
                }
                let cost = dp[i][j] + cost(j, k);
                dp[i | 1 << k][k].chmin(cost);
            }
        }
    }
    let ans = dp[(1<<n)-1][0];
    println!("{}", ans);
}
