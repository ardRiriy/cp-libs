#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        _n: usize,
        s: Chars,
    }
    let cnt = s.iter().filter(|c| c == &&'R').count();
    let ans = s[..cnt].iter().filter(|c| c == &&'W').count();
    println!("{}", ans);
}
