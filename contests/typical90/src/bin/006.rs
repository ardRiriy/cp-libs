use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars
    }

    let mut map = BTreeMap::new();
    for i in 0..n-k {
        *map.entry(s[i]).or_insert(0) += 1u64;
    }

    let mut l = 0;
    let mut ans = vec![];
    for i in n-k..n {
        *map.entry(s[i]).or_insert(0) += 1;

        // mapの最初を読む
        let (&c, &val) = map.first_key_value().unwrap();
        while s[l] != c {
            let size = *map.get(&s[l]).unwrap();
            if size == 1 {
                map.remove(&s[l]);
            } else {
                map.insert(s[l], size-1);
            }
            l += 1;
        }

        ans.push(c);
        if val == 1 {
            map.remove(&c);
        } else {
            map.insert(c, val-1);
        }
        l += 1;
    }

    println!("{}", ans.iter().join(""));
}
