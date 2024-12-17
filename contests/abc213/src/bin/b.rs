#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    println!("{}", (1..=n).sorted_unstable_by_key(|i| a[*i - 1]).collect_vec()[n-2]);
}
