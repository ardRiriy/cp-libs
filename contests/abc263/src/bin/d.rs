use proconio::input;

fn dp(a: &Vec<i64>, x: i64) -> Vec<i64> {
    let n = a.len();

    let mut dp = vec![vec![1 << 60; 2]; n+1];
    dp[0] = vec![0, 0];
    for i in 0..n {
        dp[i+1][0] = dp[i][0] + x;
        dp[i+1][1] = (dp[i][0] + a[i]).min(dp[i][1] + a[i]);
    }

    (0..=n).map(|i| dp[i][0].min(dp[i][1])).collect()
}

fn main() {
    input!{
        n: usize,
        l: i64,
        r: i64,
        a: [i64; n],
    }

    let lv = dp(&a, l);
    let rv = dp(
        &a.iter()
            .copied()
            .rev()
            .collect(),
        r)
        .iter()
        .rev()
        .copied()
        .collect::<Vec<i64>>();

    println!("{}", (0..=n).map(|i| lv[i] + rv[i]).min().unwrap());
}
