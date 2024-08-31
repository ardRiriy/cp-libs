use cps::consts::INF;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        a: [i64 ;n]
    }


    let mut dp = vec![vec![-1i64; 2]; n+1];
    dp[0][0] = 0;

    for i in 0..n {
        let exp = a[i];
        for j in 0..2usize {
            if dp[i][j] == -1 {
                continue;
            }
            let next = exp + (exp * j as i64);
            let current = dp[i][j];
            if dp[i+1][1-j] < next + current {
                dp[i+1][1-j] = next + current;
            }

            if dp[i+1][j] < current {
                dp[i+1][j] = current;
            }
        }
    }

    println!("{}", dp[n][0].max(dp[n][1]));
}
