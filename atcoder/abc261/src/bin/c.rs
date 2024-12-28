use std::collections::BTreeMap;

#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        ss: [String; n],
    }

    let mut map :BTreeMap<String, u64> = BTreeMap::new();

    for s in ss.iter() {
        let s = s.clone();
        if let Some(cnt) = map.get(&s) {
            println!("{}({})", s, cnt);
        } else {
            println!("{}", s);
        }
        *map.entry(s).or_insert(0) += 1;
    }
}

