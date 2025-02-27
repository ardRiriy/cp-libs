use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        v: [(usize, usize); n],
    }

    let mut dp = vec![vec!['-'; s+1]; n+1];
    dp[0][0] = 'S';

    for i in 0..n {
        let (a, b) = v[i];
        for j in 0..s {
            if dp[i][j] != '-' && j+a <= s {
                dp[i+1][j+a] = 'A';
            }
            if dp[i][j] != '-' && j+b <= s {
                dp[i+1][j+b] = 'B';
            }
        }
    }
    if dp[n][s] == '-' { 
        println!("Impossible");
        return;
    }
    
    let mut cur = s;
    let mut ans = vec![];
    for i in (1..=n).rev() {
        assert_ne!(dp[i][cur], '-');
        ans.push(dp[i][cur]);
        if dp[i][cur] == 'A' {
            cur -= v[i-1].0;
        } else {
            cur -= v[i-1].1;
        }
    }
    println!("{}", ans.iter().rev().join(""));
}