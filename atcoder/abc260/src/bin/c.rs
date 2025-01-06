#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        x: u64,
        y: u64, 
    }

    // dp[i][j] := level n, 色j(0ならred)の宝石
    let mut dp = vec![vec![0; 2]; n+1];

    dp[n][0] = 1;

    for i in (2..=n).rev() {
        // j = 0
        let r = dp[i][0];
        dp[i-1][0] += r;
        dp[i][1] += r * x;

        let b = dp[i][1];
        dp[i-1][0] += b;
        dp[i-1][1] += b * y;
    }
    println!("{}", dp[1][1]);
}

