use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        x: usize,
        p: [u64; n],
    }

    let p = p.iter()
        .map(|&pi| pi as f64 / 100. )
        .collect_vec();

    // Prob(i) := N個からi個のレアを得る確率
    // dp[i][j] := i個中j個のレアを得る確率として確率DP
    let mut dp = vec![vec![0.0; n+1]; n+1];
    dp[0][0] = 1.;

    for i in 0..n {
        for j in 0..=i {
            // レアを取得
            dp[i+1][j+1] += dp[i][j] * p[i];
            // レアを取得できず
            dp[i+1][j] += dp[i][j] * (1. - p[i]);
        }
    }

    let prob = &dp[n];

    // dp[i] := パックからi個以上のレア得たときの、開封したパックの個数の期待値
    let mut dp = vec![0.; x+1];

    for i in 1..=x {
        dp[i] += 1.;
        for j in 1..=n {
            // j枚レアが出た
            let nxt = if i >= j {
                i - j
            } else {
                0
            };
            dp[i] += prob[j] * dp[nxt]; 
        }
        dp[i] /= 1. - prob[0];
    }

    println!("{:10}", dp[x]); 
}