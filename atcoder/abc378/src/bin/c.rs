use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;
fn main() {
    input!{
        n: usize,
        a: [i64; n],
    }

    let mut last = BTreeMap::new();
    let mut b = vec![-1i64; n];
    for (i, &ai) in a.iter().enumerate() {
        if let Some(pos) = last.get(&ai) {
            b[i] = *pos;
        } 

        last.insert(ai, i as i64 +1);
    }
    println!("{}", b.iter().join(" "));
}
