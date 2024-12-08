// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        v: [(i128, i128); t]
    }

    println!("{}", v.iter().map(|&(a, b)| a + b).join("\n"));
}
