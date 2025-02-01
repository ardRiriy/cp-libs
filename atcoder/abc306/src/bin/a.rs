use std::iter::repeat;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        n: usize,
        s: Chars
    }
    println!("{}", s.iter().map(|&c| repeat(c).take(2).collect::<String>()).collect::<String>());
}
