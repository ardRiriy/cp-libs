#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: Chars,
    }

    let s = n;
    // dp[i][j][k] := 上からi桁目までを見たときに、i桁目に採用する値がjであるときに含まれる1の数 k=0ならNギリギリ、そうでないならN未満(自由に遷移可能)
    let mut dp = vec![vec![vec![0; 2]; 10]; s.len() + 1];

    dp[0][0][0] = 0;

    for i in 0..s.len() {
        for j in 0..10 {
            for k in 0..=1 {
                for d in 0..10 {
                    if k == 0 && d > s[i].to_digit(10).unwrap() {
                        continue;
                    }
                    let ni = i + 1;
                    let nj = d;
                    let nk = if k == 1 || d < s[i].to_digit(10).unwrap() { 1 } else { 0 };
                }
            }

        }
        dbg!(i);
        eprintln!("{}", dp[i+1].iter().map(|v| v.iter().join(" ")).join("\n"));
    }

    let ans = dp[s.len()].iter().flatten().sum::<usize>();
    println!("{}", ans);
}

