use ac_library::{Mod998244353, ModInt998244353, StaticModInt};
use proconio::input;

static MAX_VAL: usize = 3000;

fn main() {
    input!{
        n: usize,
        a: [u32; n],
        b: [u32; n]
    }

    let mut dp = vec![vec![ModInt998244353::new(0); MAX_VAL+1]; n+1];

    for i in 0..n {
        let mut sum = ModInt998244353::new(0);
        for j in 0..=b[i] as usize {
            sum += dp[i][j];
            if j >= a[i] as usize {
                if i == 0 {
                    dp[i+1][j] += 1;
                } else {
                    dp[i+1][j] = sum;
                }
            }
        }
    }

    println!("{}", dp[n].iter().sum::<StaticModInt<Mod998244353>>());
}
