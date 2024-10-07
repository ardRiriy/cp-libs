use cps::{chlibs::ChLibs, consts::INF};
use proconio::{input};
fn main() {
    input!{
        n: usize,
        x: usize,
        v: [(usize, u64, usize, u64); n]
    }

    let mut dp = vec![vec![INF; x+1]; n];

    for (i, &(a, p, b, q)) in v.iter().enumerate() {
        dp[i][0] = 0;

        // a, p
        let mut j = a;
        while j < x+1 {
            dp[i][j] = dp[i][j-a] + p;
            j += a;
        }

        // b, q
        for j in 0..x+1 {
            if dp[i][j] == INF {
                continue;
            }
            if j + b >= x + 1 {
                break;
            }

            dp[i][j+b] = dp[i][j] + q;
        }

        let mut min = INF;
        for j in (0..x+1).rev() {
            min.chmin(dp[i][j]);
            dp[i][j] = min;
        }
    }

    let mut ok = 0;
    let mut ng = x;

    while ng-ok > 1 {
        let mid = (ok + ng) / 2;
        let sum = dp.iter()
            .map(|v| v[mid.min(x)])
            .sum::<u64>();
        if sum as usize <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
