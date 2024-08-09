use std::collections::{BTreeMap, BTreeSet};

use proconio::{input};
fn solve() {
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
        return if *self > elm {
            *self = elm;
            true
        } else {
            false
        };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return if *self < elm {
            *self = elm;
            true
        } else {
            false
        };
    }
}

fn main() {
    // input! { mut i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}

