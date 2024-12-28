use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        s: [Chars; n]
    }

    let mut ans = vec![vec![]; 100];
    for i in 0..n {
        for j in 0..s[i].len() {
            ans[j].push(s[i][j]);
        }
        for j in s[i].len()..100 {
            ans[j].push('*');
        }
    }

    for i in 0..100 {
        ans[i].reverse();

        if ans[i].iter().all(|c| *c == '*') {
            continue;
        }
        while !ans[i].is_empty() && *ans[i].last().unwrap() == '*' {
            ans[i].pop();
        }
        println!("{}", ans[i].iter().join(""));
    }
}
