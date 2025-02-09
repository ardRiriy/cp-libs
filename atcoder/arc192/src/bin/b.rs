#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }
    if n == 1 {
        println!("Fennec");
    } else if n == 2 {
        println!("Snuke");
    } else if n == 3 {
        if a.iter().any(|i| *i % 2 == 1) {
            println!("Fennec");
        } else {
            println!("Snuke");
        }
    } else {
        let sum = a.iter().sum::<u64>();
        println!("{}", if sum % 2 == 0 { "Snuke" } else { "Fennec" });
    }
}

