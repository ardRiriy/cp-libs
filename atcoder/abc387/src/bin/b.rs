#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        x: u64,
    }

    let mut ans = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j != x {
                ans += i*j;
            }
        }
    }
    println!("{ans}");
}

