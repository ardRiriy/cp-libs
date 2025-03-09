#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        p: [(i64, i64); n],
    }

    let xs = p.iter()
        .map(|&(x, y)| x + y)
        .sorted()
        .collect_vec();
    let ys = p.iter()
        .map(|&(x, y)| x-y)
        .sorted()
        .collect_vec();
    println!("{}", (xs[n-1]-xs[0]).max(ys[n-1]-ys[0]));
}

