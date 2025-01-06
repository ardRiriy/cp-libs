use std::cmp::Reverse;

#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [u64; n],
        b: [u64; n],
    }
    let mut ans = vec![false; n];
    let a_indicate = (0..n)
        .sorted_by_key(|i| Reverse(a[*i]))
        .collect_vec();

    for i in a_indicate[..x].iter() {
        ans[*i] = true;
    }

    let b_indicate = (0..n)
        .filter(|i| !ans[*i])
        .sorted_by_key(|i| Reverse(b[*i]))
        .collect_vec();
    for i in b_indicate[..y].iter() {
        ans[*i] = true;
    }

    let s = (0..n).map(|i| a[i] + b[i]).collect_vec();
    let s_indicate = (0..n)
        .filter(|i| !ans[*i])
        .sorted_by_key(|i| Reverse(s[*i]))
        .collect_vec();
    for i in s_indicate[..z].iter() {
        ans[*i] = true;
    }

    println!("{}", (1..=n).filter(|i| ans[*i-1]).join("\n"));
}

