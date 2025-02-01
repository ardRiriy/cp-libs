#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn solve() {
    input! {
        n: u64,
        k: usize,
    }
    let mut tmp = vec![];
    let mut v = vec![];
    for i in 0..64 {
        if (n>>i) & 1 == 0 {
            tmp.push(i);
        } else {
            for &ti in tmp.iter() {
                v.push(ti);
            }
            tmp.clear();
        }
    }
    let z = v.len();
    let z2 = 2usize.pow(z as u32);
    if z2 < k {
        println!("-1");
        return;
    }

    let mut r = 0;
    for (i, &vi) in v.iter().enumerate() {
        r += ((k-1)>>i & 1) * 2usize.pow(vi as u32);
    }
    println!("{}", n+r as u64);
}

fn main() {
    input!{
        t: usize,
    }
    for _ in 0..t {
        solve();
    }
}


