use std::{
    collections::{BTreeMap, VecDeque},
    iter::repeat,
};

#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
    }

    let v = (0..m)
        .map(|_| {
            input! {
                k: usize,
                a: [u64; k],
            }
            a
        })
        .collect_vec();

    let mut indicates = repeat(0).take(m).collect_vec();

    let mut que = VecDeque::from_iter((0..m).into_iter());
    let mut pool = BTreeMap::new();

    while let Some(p) = que.pop_front() {
        if let Some(&other) = pool.get(&v[p][indicates[p]]) {
            let other: usize = other;
            pool.remove(&v[p][indicates[p]]);

            indicates[p] += 1;
            indicates[other] += 1;

            if indicates[p] != v[p].len() {
                que.push_back(p);
            }

            if indicates[other] != v[other].len() {
                que.push_back(other);
            }
        } else {
            pool.insert(v[p][indicates[p]], p);
        }
    }

    println!(
        "{}",
        if indicates
            .iter()
            .enumerate()
            .all(|(i, idx)| v[i].len() == *idx)
        {
            "Yes"
        } else {
            "No"
        }
    );
}
