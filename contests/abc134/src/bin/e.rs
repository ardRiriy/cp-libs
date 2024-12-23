use std::cmp::Reverse;

use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }

    let mut bit = FenwickTree::new(n, 0u64);
    let mut ans = 0;
    
    for (idx, &x) in a.iter().enumerate().sorted_by_key(|x| (x.1, Reverse(x.0))) {
        let val = bit.sum(0..idx);
        if val == 0 {
            ans += 1;
        }
        bit.add(idx, 1);
    }
    println!("{}", ans);
}

