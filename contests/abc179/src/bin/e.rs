#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: u64,
        x: u64,
        m: u64,
    }

    let mut cs = vec![0];
    let mut seen = vec![false; usize::try_from(m).unwrap()];
    let mut current = x;

    while !seen[current] {
        seen[current] = true;
        let val = cs.last().unwrap() + current;
        cs.push(current);

        current = (current * current) % m;
    }

    let cycle_len = cs.len() - 1;
}

