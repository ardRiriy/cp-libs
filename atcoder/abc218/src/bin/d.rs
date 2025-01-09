use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;
fn main() {
    input!{
        n: usize,
        p: [(u64, u64); n],
    }

    let mut map = BTreeMap::new();
    for (&(ax, ay), &(bx, by)) in p.iter().tuple_combinations() {
        if ax == bx {
            let a = ay.min(by);
            let b = ay.max(by);
            *map.entry((a, b)).or_insert(0) += 1u64;
        }
    }

    fn sum(n: u64) -> u64 {
        n * (n+1) / 2
    }
    
    let mut ans = 0;
    for (_, val) in map {
        ans += sum(val-1);
    }
    println!("{}", ans);
}

