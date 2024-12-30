#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [u64; n],
        b: [u64; m],
    }
    let inf = 1u64 << 60;
    let mut dp = vec![vec![inf; m+1]; n+1];

    dp[0][0] = 0;
    for i in 0..=n {
        dp[i][0] = i as u64;
    }
    for i in 0..=m {
        dp[0][i] = i as u64;
    }

    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = (dp[i-1][j-1] + if a[i-1] == b[j-1] { 0 } else { 1 }).min(dp[i-1][j] + 1).min(dp[i][j-1]+1);
        }
    }
    println!("{}", dp[n][m]);
}

