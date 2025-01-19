#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        x: u64,
    }
    let mut a = 1;
    for i in 1.. {
        a *= i;
        if a == x {
            println!("{}", i);
            return;
        }
    } 
}

