use itertools::Itertools;
use proconio::input;
use ac_library::ModInt998244353 as Mint;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut dp = vec![vec![vec![Mint::new(0); n+1]; n]; n];

    for i in (0..n).rev() {
        for j in i+1..n {
            dp[i][j][2] += 1; // 長さ2
            for l in 2..n-i {
                for k in j+1..n {
                    if a[k] - a[j] != a[j] - a[i] {
                        continue;
                    }
                    let val = dp[j][k][l];
                    dp[i][j][l+1] += val;
                }
            }
        }
    }
    let mut ans = vec![Mint::new(0); n];
    ans[0] = Mint::new(n);
    for i in 0..n {
        for j in 0..n {
            for k in 2..=n {
                ans[k-1] += dp[i][j][k];
            }
        }
    }
    println!("{}", ans.iter().join(" "));

}