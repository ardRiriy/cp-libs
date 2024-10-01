use std::collections::BTreeMap;

use ac_library::ModInt998244353;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut map = BTreeMap::new();
    map.insert(0, ModInt998244353::new(1));
    let mut all = ModInt998244353::new(1);
    let mut acc = 0;

    for (i, x) in a.iter().enumerate() {
        acc += *x;
        let t = match map.get(&(acc-k)) {
            Some(x) => *x,
            None => ModInt998244353::new(0),
        };
        let current = all - t;
        *map.entry(acc).or_insert(ModInt998244353::new(0)) += current;
        all += current;

        if i == n-1 {
            println!("{}", current);
        }
    }
}
