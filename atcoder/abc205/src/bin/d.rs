#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        q: usize,
        mut a: [u64; n],
        queries: [u64; q],
    }
    a.insert(0, 0);

    let indicates = (0..q)
        .sorted_unstable_by_key(|i| queries[*i])
        .collect_vec();
    
    

}

