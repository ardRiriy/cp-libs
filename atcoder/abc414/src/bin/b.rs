use std::iter::repeat;

use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        c: [(char, usize); n],
    }
    let mut ans = vec![];
    let mut count = 0;
    for (v, c) in c.iter() {
        if count + *c > 100 {
            println!("Too Long");
            return;
        }
        count += *c;
        ans.push(repeat(*v).take(*c).collect::<String>());
    }
    println!("{}", ans.iter().join(""));
}

