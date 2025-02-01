#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut cur = 0;
    for i in a.iter().sorted_unstable() {
        if cur == *i {
            cur += 1;
        }
    }
    println!("{cur}");
}
