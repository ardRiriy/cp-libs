use std::{collections::BTreeSet, iter::FromIterator};

use proconio::{input, marker::Usize1};
fn main() {
    input!{
        h: usize,
        w: usize,
        n: usize,
        coins: [(Usize1, Usize1); n]
    }
    let set = BTreeSet::from_iter(coins);
}

