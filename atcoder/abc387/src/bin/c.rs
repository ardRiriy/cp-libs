#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn to_i(c: char) -> usize {
    c as usize - '0' as usize
}

fn count(x: u64) -> u64 {
    // x未満で条件を満たすものの個数
    // dp[i][j][k] := i桁目まで見たときに、頭の桁がjであり、
    // if k = 1 then x未満であることが確定している
    // else x未満になる可能性がある

    let xs = x.to_string().chars().collect_vec();
    let len = x.to_string().len();
    let mut dp = vec![vec![vec![0u64; 2]; 10]; len+1];

    dp[0][0][0] = 1;

    for i in 0..len {
        if i != len-1 {
            // j = 0 && k = 0
            for nj in 0..to_i(xs[i]) {
                dp[i+1][nj][1] += dp[i][0][0];
            } 
            dp[i+1][to_i(xs[i])][0] += dp[i][0][0];

            // j = 0 && k = 1
            for nj in 0..=9 {
                dp[i+1][nj][1] += dp[i][0][1];
            }
        }

        // k = 0
        for j in 1..=9 {
            if to_i(xs[i]) < j {
                dp[i+1][j][0] += dp[i][j][0];
            }

            for _ in 0..to_i(xs[i]).min(j) {
                dp[i+1][j][1] += dp[i][j][0];
            }
        }

        // k = 1
        for j in 1..=9 {
            let val = dp[i][j][1] * j as u64;
            dp[i+1][j][1] += val; 
        }
    }

    dp[len].iter().flatten().sum()
}

fn main() {
    input!{
        l: u64,
        r: u64,
    }
    println!("{}", count(r) - count(l-1));

}

