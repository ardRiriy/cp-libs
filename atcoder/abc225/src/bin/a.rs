use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        s: Chars,
    }

    let mut set = BTreeSet::new();
    for v in s.iter().copied().permutations(s.len()) {
        set.insert(v);
    }
    println!("{}", set.len());
}

