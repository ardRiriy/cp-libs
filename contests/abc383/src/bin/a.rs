#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        v: [(usize, u64); n],
    }
    let mut t = 0;
    let mut i = 0;
    let mut l = 0;
    while i < n {
        if l != 0 {
            l -= 1;
        }
        if t == v[i].0 {
            l += v[i].1;
            i += 1;
        }
        t += 1;
    }
    println!("{l}");
}
