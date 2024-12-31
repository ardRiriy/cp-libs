use cps::cumulative_sum::CumulativeSum;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;
use ac_library::ModInt1000000007 as mint;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }

    let csum = CumulativeSum::new(&a);
    let mut dp = vec![vec![mint::new(0); n+2]; n+2];
    dp[1][0] += 1;
    let mut ans = mint::new(0);
    for i in 0..n {
        for j in (1..=n).rev() {
            let val = dp[j][csum.get(..i+1)as usize%j];
            dp[j+1][csum.get(0..i+1) as usize%(j+1)] += val;
            if i == n-1 {
                ans += dp[j][csum.get(..i+1)as usize%j];
            }
        }
    }
    println!("{}", ans);
}

