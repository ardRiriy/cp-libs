#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        d: u64,
        v: [(u64, u64); n],
    }

    for i in 1..=d {
        println!("{}", v.iter().map(|&(t, d)| t*(d+i)).max().unwrap());
    }
}

