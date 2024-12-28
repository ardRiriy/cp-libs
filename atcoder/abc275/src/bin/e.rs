use ac_library::{ModInt998244353, StaticModInt};
use proconio::{input};
fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![vec![ModInt998244353::new(0); n+1]; k+1];
    dp[0][0] = ModInt998244353::new(1);
    let p = ModInt998244353::new(1) / m;

    for i in 0..k {
        for j in 0..n {
            let mut is_add = true;
            let mut index = j;
            for _ in 1..=m {
                // ターンiにマスjからkマス進む確率はp
                if is_add {
                    index += 1;
                } else {
                    index -= 1;
                }
                if index == 0 {
                    is_add = true;
                } else if index == n {
                    is_add = false;
                }
                let val = dp[i][j] * p;
                dp[i+1][index] += val;
            }
        }
    }
    let ans = dp.iter()
        .map(|v| v[n])
        .sum::<ModInt998244353>();
    println!("{}", ans);
}
