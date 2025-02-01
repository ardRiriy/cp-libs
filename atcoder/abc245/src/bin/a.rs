#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    if a > c || (a == c && b > d) {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}
