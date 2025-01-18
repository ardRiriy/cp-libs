use std::collections::{BTreeMap, BTreeSet};

use proconio::input;
fn main() {
    input!{
        n: usize,
        mut k: i64,
        a: [i64; n]
    }

    let mut sum = 0;
    let mut set = BTreeSet::new();
    let mut map = BTreeMap::new();

    for x in a.iter() {
        sum += *x;
        set.insert(sum);
        *map.entry(sum).or_insert(0u64) += 1;
    }

    let mut ans = 0;
    sum = 0;
    for x in a.iter() {
        sum += *x;
        if set.contains(&k) {
            ans += map[&k];
        }

        let t = map[&sum];
        map.insert(sum, t-1);
        if t == 1 { set.remove(&sum); }

        k += *x;
    }

    println!("{ans}");
}
