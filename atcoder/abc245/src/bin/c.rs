#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    }

    let v = vec![a, b];
    let mut dp = vec![vec![false; 2]; 2];
    let mut cur = 0;
    dp[cur] = vec![true, true];

    for i in 0..n - 1 {
        let nxt = 1 - cur;

        for j in 0..2 {
            if !dp[cur][j] {
                continue;
            }
            for l in 0..2 {
                dp[nxt][l] = dp[nxt][l] || (v[l][i + 1] - v[j][i]).abs() <= k;
            }
        }
        dp[cur] = vec![false, false];
        cur = nxt;
    }

    println!(
        "{}",
        if dp[cur].iter().any(|n| *n) {
            "Yes"
        } else {
            "No"
        }
    );
}
