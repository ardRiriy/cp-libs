// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        v: [(u64, u64); t],
    }

    println!("{}", v.iter().map(|(a, b)| a + b).join("\n"));
}
