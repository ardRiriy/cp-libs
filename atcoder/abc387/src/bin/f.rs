#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [Usize1; n],
    }

    // i -> aiであるfuncional graph
    let mut g = vec![vec![]; n];
    let mut rev_g = vec![vec![]; n];
    for (i, ai) in a.iter().enumerate() {
        g[i].push(*ai);
        rev_g[*ai].push(i);
    }

    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }

        
    }

}

