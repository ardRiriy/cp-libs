#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }
    let v = a[0..n/2].iter().copied().collect_vec();
    let w = a[n/2..].iter().copied().collect_vec();
    let mut vi = 0;
    let mut wi = 0;

    let mut ans = 0;
    while vi < v.len() && wi < w.len() {
        if v[vi]*2 <= w[wi] {
            ans += 1u64;
            vi += 1;
            wi += 1;
        } else {
            wi += 1;
        }
    }
    println!("{}", ans);
}

