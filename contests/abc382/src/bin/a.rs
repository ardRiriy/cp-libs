#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        _n: usize,
        d: usize,
        s: Chars,
    }

    let count = s.iter().filter(|&c| *c == '.').count();
    println!("{}", count + d);
}
