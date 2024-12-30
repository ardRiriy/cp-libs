#[allow(unused_imports)]
use cps::debug::*;
use num::pow;
use num_integer::Roots;
use proconio::input;

fn main() {
    input!{
        x: i64,
    }

    for b in 0.. {
        for k in [-1, 1] {
            let a_sub = pow(b*k, 5) + x;
            let a = a_sub.nth_root(5);
            if pow(a, 5) == a_sub {
                println!("{} {}", a, b*k);
                return;
            }
        }
    }
}

