use cps::chlibs::ChLibs;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        x: usize,
        k: i64,
        mut items: [(usize, i64, Usize1); n],
    }
    let items = items.iter()
        .fold(vec![vec![]; n], |mut v, &(p, u, c)| {
            v[c].push((p, u));
            v
    });
    let inf :i64 = -1;
    let mut dp =vec![vec![inf; x+1]; 2];
    let mut cur = 0;
    dp[cur][0] = 0;
    for v in items.iter() {
        if v.is_empty() {
            continue;
        }
        let nxt = 1 - cur;
        for &(p, u) in v.iter() {
            for i in (0..=x).rev() {
                if dp[cur][i] != inf {
                    // 買わない
                    let val = dp[cur][i];
                    dp[nxt][i].chmax(val);
                }
                if i + p > x {
                    continue;
                }

                // cur -> nxt (k)
                if dp[cur][i] != inf {
                    let val = dp[cur][i] + u + k;
                    dp[nxt][i+p].chmax(val);

                }
                // nxt -> nxt
                if dp[nxt][i] != inf {
                    let val = dp[nxt][i] + u;
                    dp[nxt][i+p].chmax(val);
                }
            }
        }
        dp[cur].fill(inf);
        cur = nxt;
    }
    
    println!("{}", dp[cur].iter().max().unwrap());
}

