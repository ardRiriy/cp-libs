use std::collections::{BTreeMap, BTreeSet};
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;
use proconio::marker::Chars;
use cps::chlibs::ChLibs;
use itertools::Itertools;

fn solve(n: usize, a: &Vec<Vec<char>>, p: u128) -> u64{

    let a = a.iter()
        .map(|v| {
            let mut base = 1u128;
            let mut val = 0;
            for c in v.iter().rev() {
                val += base * c.to_digit(10).unwrap() as u128;
                val %= p;
                base *= 10;
                base %= p;
            }
            val
        }).collect_vec();
    let mut map = BTreeMap::new();
    for val in a.iter() {
        *map.entry(*val).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans += if let Some(val) = map.get(&((a[i] * a[j])%p)) {
                *val
            } else {
                0
            };
        }
    }
    ans
}

fn main() {
    input!{
        n: usize,
        a: [Chars; n],
    }
    let primes = vec![
        134151471827385811,
        309033273316739033,
        776892570074781649,
    ];

    let mut count = BTreeMap::new();
    for &p in &primes {
        let res = solve(n, &a, p);
        *count.entry(res).or_insert(0) += 1;

        if let Some(val) = count.get(&res) {
            if *val > primes.len() / 2 {
                println!("{}", res);
                return;
            }
        }
    }

    let mut mx = 0;
    let mut ans = 0;
    for (&k, &v) in count.iter() {
        if mx.chmax(v) {
            ans = k;
        }
    }
    println!("{}", ans);
}


