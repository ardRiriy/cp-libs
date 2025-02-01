use cps::chlibs::ChLibs;
use proconio::input;

fn main() {
    input!{
        n: usize,
    }
    let mut dp = vec![vec![0; 2]; 2];
    let mut cur = 0;

    for _ in 0..n {
        let nxt = 1 - cur;
        input! {
            x: u8,
            y: i64,
        }
        // 食べる
        if x == 0 {
            for i in 0..=1 {
                let val = dp[cur][i] + y;
                dp[nxt][0].chmax(val);
            }
        } else {
            let val = dp[cur][0] + y;
            dp[nxt][1].chmax(val);
        }

        // 下げる
        for i in 0..=1 {
            let val = dp[cur][i];
            dp[nxt][i].chmax(val);
        }

        dp[cur].fill(0);
        cur = nxt;
    }
    println!("{}", dp[cur].iter().max().unwrap());
}

