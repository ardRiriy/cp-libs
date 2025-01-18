#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn maximam(b: &Vec<i64>) -> i64 {
    // dp[i][j] := i番目までの総和の最大値
    // j = 1の時選択必須
    let n = b.len();
    let inf = -1e18 as i64;
    let mut dp = vec![vec![inf; 2]; n+1];
    dp[0] = vec![0, 0];
    for i in 0..n {
        // 選ぶ
        for j in 0..=1 {
            dp[i+1][0] = dp[i+1][0].max(dp[i][j]+b[i]);
        }

        // 選ばない
        dp[i+1][1] = dp[i][0];
    }

    *dp[n].iter().max().unwrap()
}

fn main() {
    input!{
        n: usize,
        a: [i64; n],
    }
    {
        // avg
        let a = a.iter().map(|&ai| ai * 1e4 as i64).collect_vec();

        let mut ok = 1;
        let mut ng = 1e9 as i64 * 1e4 as i64 * 10;
        while ng-ok>1{
            let mid = (ok+ng)/2;
            let b = a.iter().map(|&ai| ai - mid).collect_vec();
            if maximam(&b) >= 0 {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok as f64 / 1e4);
    }
    {
        // median
        let mut ok = 1;
        let mut ng = 1e9 as i64 * 10;
        while ng-ok>1 {
            let mid = (ng+ok)/2;
            let b = a.iter().map(|&ai| if ai >= mid { 1 } else { -1 }).collect_vec(); 
            if maximam(&b) > 0 {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok);
    }

}

