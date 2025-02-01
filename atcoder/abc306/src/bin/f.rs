use std::collections::BTreeMap;
use ac_library::FenwickTree;
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [[u64; m]; n],
    }
    let mut map = BTreeMap::new();
    for (i, ai) in a.iter().flatten().sorted_unstable().enumerate() {
        map.insert(*ai, i);
    }

    let mut bit = FenwickTree::new(n*m, 0);
    let mut ans = 0u64;

    for (i, v) in a.iter().enumerate().rev() {
        for (j, &aij) in v.iter().sorted_unstable().enumerate().rev() {
            let idx = *map.get(&aij).unwrap();
            ans += ((j+1) * (n-i-1)) as u64 + bit.sum(0..idx);
            bit.add(idx, 1);
        }
    }
    println!("{}", ans);
}

