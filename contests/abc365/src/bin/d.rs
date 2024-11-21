use cps::chlibs::ChLibs;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        n: usize,
        s: Chars,
    }

    // aが勝ったらtrue
    fn is_win(a: char, b: char) -> bool {
        if a == 'R' && b == 'S' {
            true
        } else if a == 'S' && b == 'P' {
            true
        } else { a == 'P' && b == 'R' }
    }

    let te = ['R', 'S', 'P'];
    let mut dp = vec![vec![0u64; 3];n+1];
    for i in 0..n {
        for j in 0..3 {
            let mut val = 0;
            for k in 0..3 {
                if j == k { continue; }
                if is_win(s[i], te[k]) { continue; }
                val.chmax(dp[i][k] + if is_win(te[k], s[i]) { 1 } else { 0 });
            }

            dp[i+1][j].chmax(val);
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
