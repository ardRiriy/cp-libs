use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        ss: [Chars; n],
    }
    println!("{}", ss.iter().sorted_unstable_by_key(|s|s.len()).map(|v|v.iter().join("")).join(""));
}

