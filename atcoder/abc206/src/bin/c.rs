use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }
    let mut map = BTreeMap::new();
    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (_, v) in map {
        ans += v * (n-v);
    }
    println!("{}", ans/2);
}

