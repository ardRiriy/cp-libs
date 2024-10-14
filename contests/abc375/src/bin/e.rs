use cps::chlibs::ChLibs;
use cps::consts::INF;
use proconio::input;
fn main() {
    input!{
        n: usize,
        v: [(usize, usize); n],
    }
    let sum = v.iter()
        .map(|(_, b)| *b)
        .sum::<usize>();

    if sum % 3 != 0 {
        println!("-1");
        return;
    }

    let target = sum / 3;

    let mut dp = vec![vec![vec![INF; target + 1]; target + 1]; n+1];
    dp[0][0][0] = 0;

    for (idx, &(a, b)) in v.iter().enumerate() {
        for i in (0..=target).rev() {
            for j in( 0..=target).rev() {
                if dp[idx][i][j] == INF {
                    continue;
                }
                let val = dp[idx][i][j];

                if i + b <= target {
                    dp[idx+1][i + b][j].chmin(val + if a == 1 { 0 } else { 1 });
                }
                if j + b <= target {
                    dp[idx+1][i][j + b].chmin(val + if a == 2 { 0 } else { 1 });
                }
                dp[idx+1][i][j].chmin(val + if a == 3 { 0 } else { 1 });


            }
        }
    }

    if dp[n][target][target] == INF {
        println!("-1");
    } else {
        println!("{}", dp[n][target][target]);
    }
}
