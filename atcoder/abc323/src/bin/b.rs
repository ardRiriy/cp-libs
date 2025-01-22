use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let ans = s
        .iter()
        .enumerate()
        .map(|(i, v)| (v.iter().filter(|&c| *c == 'o').count(), i))
        .sorted_by_key(|&(c, i)| (Reverse(c), i))
        .map(|(_, i)| i + 1)
        .join(" ");
    println!("{ans}");
}
