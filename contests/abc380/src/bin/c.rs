use std::iter::repeat;

use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    }

    let mut ran_length = s.iter().dedup_by_with_count(|a, b| a == b).collect_vec();
    if s[0] == '0' {
        ran_length.swap(k * 2 - 1, k * 2 - 2);
    } else {
        ran_length.swap(k * 2 - 2, k * 2 - 3);
    }
    println!(
        "{}",
        ran_length
            .iter()
            .map(|&(k, &c)| repeat(c).take(k).join(""))
            .join("")
    );
}
