use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn solve() {
    input! {
        n: usize,
    }

    let mut num = vec![];
    for _ in 0..n {
        input!{
            m: usize,
            v: [(u64, u64); m]
        }
        num.push(v);
    }

    let mut map:BTreeMap<u64, (u64, u64)> = BTreeMap::new();

    for v in num.iter() {
        for &(p, e) in v {
            let mut current = match map.get(&p) {
                Some(x) => *x,
                None => (0, 0), 
            };

            if current.0 < e{
                current.1 = current.0;
                current.0 = e;
            } else if current.1 < e {
                current.1 = e;
            }
            map.insert(p, current);
        }
    }

    let mut set = BTreeSet::new();
    for v in num.iter() {
        let mut a = vec![];
        for &(p, e) in v {
            let t = map[&p];
            if t.0 == e && t.0 != t.1 {
                a.push(p);
            }
        }
        set.insert(a);
    }
    println!("{}", set.len());

}

/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/


static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        if *self > elm {
                *self = elm;
                true
            } else { false }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
                *self = elm;
                true
            } else { false }
    }
}


fn main() {
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
