#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        c: Chars
    }
    println!("{}UPC" , c[0]);
}
