use num::Float;
use proconio::input;

fn main() {
    input!{
        n: usize,
        p: [f64; n],
    }

    const INF :f64 = -1e18;
    let mut dp = vec![vec![INF; n+1]; n+1];
    dp[n][0] = 0.;

    for i in (0..n).rev() {
        for j in 0..n {
            dp[i][j] = dp[i][j].max(dp[i+1][j]);
            dp[i][j+1] = dp[i][j+1].max(dp[i+1][j] + p[i] * 0.9.powf(j as f64));
        }
    }

    let mut ans = INF;
    let mut div = 0.;
    for i in 1..=n {
        div += 0.9.powf((i-1) as f64);
        ans = ans.max(dp[0][i] / div - 1200. / (i as f64).sqrt());
    }
    println!("{}", ans);

}

