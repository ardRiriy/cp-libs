use ac_library::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![vec![Mint::new(0); m + 1]; n + 1];
    for i in 1..=m {
        dp[1][i] = Mint::new(1);
    }
    for i in 1..n {
        let mut imos = vec![Mint::new(0); m + 1];
        for j in 0..=m {
            if j > k && j + 1 - k <= m {
                imos[1] += dp[i][j];
                imos[j - k + 1] -= dp[i][j];
            }
            if j + k <= m {
                imos[j + k] += dp[i][j];
            }
        }
        let mut csum = Mint::new(0);
        for j in 0..=m {
            csum += imos[j];
            imos[j] = csum;
        }
        dp[i + 1] = imos;
    }
    println!("{}", dp[n].iter().sum::<Mint>());
}
