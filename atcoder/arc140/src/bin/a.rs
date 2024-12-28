#[allow(unused_imports)]
use cps::debug::*;
use cps::prime::divisors;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn check(k: usize, divisor: usize, s: &[char]) -> bool {
    let sv = s.chunks(divisor)
        .into_iter()
        .collect_vec();

    let mut v = vec![vec![0; 26]; divisor];
    for cv in sv.iter() {
        for (i, &c) in cv.iter().enumerate() {
            v[i][(c as u8 - b'a') as usize] += 1;
        }
    }

    let m = s.len() / divisor;
    v.iter().map(|v| m - v.iter().map(|x| *x).max().unwrap()).sum::<usize>() <= k
}

fn main() {
    input!{
        n: usize,
        k: usize,
        s: Chars,
    }

    let v = divisors(n as u64)
        .iter()
        .map(|x| *x as usize)
        .sorted()
        .collect_vec();

    let mut s= s.clone();
    for i in 0..n {
        s.push(s[i]);
    }


    for i in 0..n {
        for x in &v {
            if check(k, *x, &s[i..i+n]) {
                println!("{}", x);
                return;
            }
        }
    }
    unreachable!()
}
