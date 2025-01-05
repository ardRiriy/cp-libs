#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        s: Chars,
    }

    let cc = vec!['A', 'B', 'C'];
    let mut stk = vec![];
    for c in s {
        stk.push(c);
        loop {
            if stk.len() >= 3 && stk[stk.len()-3..].iter().enumerate().all(|(i, c)| cc[i] == *c) {
                for _ in 0..3 {
                    stk.pop();
                }
            } else {
                break;
            }
        }
    }
    println!("{}", stk.iter().join(""));
}

