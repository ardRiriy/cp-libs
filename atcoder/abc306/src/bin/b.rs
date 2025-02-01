#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    static N: usize = 64;
    input!{
        a: [u64; N],
    }
    let mut ans = 0u64;
    for (i, &ai) in a.iter().enumerate() {
        ans = ans | ai << i;
    }
    println!("{}", ans);
}

