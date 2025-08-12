#[allow(unused_imports)]
use proconio::input;

fn ex() {
    let a = 7;
    let b = 5;

    for x in 1..=100 {
        println!("x={x}, f(x)={}", (a*x)/b - a*(x/b));
    }
}

fn main() {
    input!{
        a: u64,
        b: u64,
        n: u64,
    }

    let x =  (b-1).min(n);
    println!("{}", (a*x)/b - a*(x/b));
}


