use ac_library::ModInt1000000007 as Mint;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
    }

    let mut dp = vec![vec![Mint::new(0); 4]; 2];
    let mut cur = 0;
    dp[cur][0] = Mint::new(1);

    for _ in 0..n {
        let nxt = 1 - cur;
        for j in 0..4 {
            let val = dp[cur][j];
            dp[nxt][j] += val*8;
            dp[nxt][j | 1] += val;
            dp[nxt][j | 1 << 1] += val;
        }
        dp[cur].fill(Mint::new(0));
        cur = nxt;
    }
    println!("{}", dp[cur][3]);
}

