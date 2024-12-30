use std::collections::BTreeSet;

#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        k: usize,
    }

    let mut set = BTreeSet::from_iter((0..n).into_iter());
    for _ in 0..k {
        input! {
            d: usize,
            a: [Usize1; d],
        }

        for i in a {
            set.remove(&i);
        }
    }
    println!("{}", set.len());
}

