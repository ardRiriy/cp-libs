use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut v: [[i64; n]; 3]
    }
    fn func(a: i64, b:i64, c:i64) -> i64 {
         a*b + b*c + c*a
    }
    for i in 0..3 {
        v[i].sort();
    }

    let mut seen = BTreeSet::new();
    let mut heap = BinaryHeap::new();

    let mut s = func(v[0][n-1],v[1][n-1], v[2][n-1]);

    let mut count = k;
    seen.insert(vec![n-1, n-1, n-1]);
    heap.push((s, vec![n-1, n-1, n-1]));

    while let Some((sum, vec)) = heap.pop() {
        count -= 1;
        if count == 0 {
            println!("{}", sum);
            return;
        }
        let mut new_v = vec;
        for i in 0..3 {
            if new_v[i] != 0 {
                new_v[i] -= 1;
                if seen.insert(new_v.clone()) {
                    heap.push((func(v[0][new_v[0]], v[1][new_v[1]], v[2][new_v[2]]), new_v.clone()));
                }
                new_v[i] += 1;
            }
        }
    }

}

