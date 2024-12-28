use cps::chlibs::ChLibs;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; k],
    }

    let mut dp = vec![0; n+1];
    dp[0] = 0;

    for i in 1..=n {
        let mut val = 0;
        for &ni in a.iter() {
            if i < ni {
                continue;
            }
            val.chmax(i - dp[i - ni]);
        }
        dp[i] = val;
    }
    println!("{}", dp[n]);
}
