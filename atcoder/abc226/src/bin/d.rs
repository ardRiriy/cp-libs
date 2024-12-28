use std::collections::BTreeSet;

use cps::*;
use itertools::Itertools;
use num_integer::gcd;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        v: [(i64, i64); n],
    }
    let mut ans = BTreeSet::new();

    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            let d = (v[i].0 - v[j].0, v[i].1 - v[j].1);

            let val = gcd(d.0, d.1);
            ans.insert((d.0 / val, d.1 / val));
        }
    }

    println!("{}", ans.len());
}

