#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        _n: usize,
        c: char,
        d: char,
        s: Chars,
    }

    let ans = s.iter().map(|&s| if s == c { s } else { d }).collect::<String>();
    println!("{}", ans);
}
