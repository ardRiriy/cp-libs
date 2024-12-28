use std::collections::BTreeSet;

#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }
    let ans = a.iter()
        .fold(BTreeSet::new(), |mut s :BTreeSet<u64>, &ai|{
            if !s.insert(ai) {
                s.remove(&ai);
            }
            s
        });
    println!("{}", ans.len());
}

