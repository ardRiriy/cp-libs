use ac_library::ModInt998244353;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![ModInt998244353::new(0); 10]; n];
    dp[0][a[0]] = ModInt998244353::new(1);

    for i in 1..n {
        for j in 0..=9 {
            dp[i][(j + a[i]) % 10] = dp[i][(j + a[i]) % 10] + dp[i-1][j];
            dp[i][(j * a[i]) % 10] = dp[i][(j * a[i]) % 10] + dp[i-1][j];
        }
    }

    println!("{}", dp[n-1].iter().join("\n"));
}

