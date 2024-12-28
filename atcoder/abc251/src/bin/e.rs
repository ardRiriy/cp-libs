use cps::consts::INF;
use proconio::input;

fn dp_f(a: &[u64], dp: &mut Vec<Vec<u64>>) {
    let n = a.len();
    for i in 1..n {
        // dp[i][0] -> dp[i+1][1]
        let val = dp[i][1].min(dp[i-1][0] + a[i]);
        dp[i][1] = val;

        // dp[i][1] -> どっちでもOK
        dp[i][0] = dp[i][0].min(dp[i-1][1]);
        dp[i][1] = dp[i][1].min(dp[i-1][1] + a[i]);
    }
}

fn main() {
    input!{
        n: usize,
        a: [u64; n]
    }

    // dp[i][j] := i番目の行動で選択jを選んだときにあり得る最小値
    // j = 0: 直前には上げていない 
    // j = 1: 直前に上げた

    // a1で上げたとするパターン
    let mut dp1 = vec![vec![INF; 2]; n];
    dp1[0][1] = a[0];

    dp_f(&a, &mut dp1);

    // a1で上げてないとするパターン
    let mut dp2 = vec![vec![INF; 2]; n];
    dp2[0][0] = 0;
    dp_f(&a, &mut dp2);

    println!("{}", dp1[n-1][0].min(dp1[n-1][1]).min(dp2[n-1][1]));
}
