use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{input};
fn main() {
    input!{
        mut s: Chars,
        t: Chars,
    }
    let mut ans = vec![];
    for i in 0..s.len() {
        if s[i] > t[i] {
            s[i] = t[i];
            ans.push(s.iter().join(""));
        }
    }

    for i in (0..s.len()).rev() {
        if s[i] != t[i] {
            s[i] = t[i];
            ans.push(s.iter().join(""));
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join("\n"));
}
