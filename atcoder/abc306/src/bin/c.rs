#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        a: [Usize1; 3*n],
    }
    let mut seen = vec![0; n];
    let mut ans = vec![];
    for ai in a {
        if seen[ai] == 1 {
            ans.push(ai+1);
        }
        seen[ai] += 1;
    }
    println!("{}", ans.iter().join(" "));
}

