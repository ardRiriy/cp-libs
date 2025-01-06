#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        _n: usize,
        s: Chars,
    }
    if !s.iter().any(|&c| c == 'x') && s.iter().any(|&c| c == 'o') {
        println!("Yes");
    } else {
        println!("No");
    }
}
