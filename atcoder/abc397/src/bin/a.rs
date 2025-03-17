#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        x: f64,
    }
    if(x<37.5) {
        println!("3");   
    } else if(x<38.) {
        println!("2");
    } else {
        println!("1");
    }
}
