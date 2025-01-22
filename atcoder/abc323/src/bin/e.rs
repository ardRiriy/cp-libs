use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: [usize; n]
    }

    let mut dp = vec![vec![ac_library::ModInt998244353::new(0); n]; x + 2];
    dp[0][0] = ac_library::ModInt998244353::new(1);

    for time in 0..=x {
        let s = dp[time].iter().sum::<ac_library::ModInt998244353>() / n;
        for (idx, &length) in t.iter().enumerate() {
            dp[(time + length).min(x + 1)][idx] += s;
        }
    }
    println!("{}", dp[x + 1][0]);
}
