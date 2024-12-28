use std::collections::BTreeMap;

#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p: [u64; m+1],
    }

    let mut map = BTreeMap::new();
    for i in 0..m {
        map.insert(d[i].clone(),p[i+1]);
    }
    let ans :u64 = c.iter()
        .map(|s| 
            if let Some(v) = map.get(s) {
                *v
            } else {
                p[0]
            }
        )
        .sum();

    println!("{ans}");
}

