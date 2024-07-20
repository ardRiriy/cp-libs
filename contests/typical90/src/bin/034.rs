use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn solve() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n]
    }

    let mut set = BTreeSet::new();
    let mut map = BTreeMap::new();

    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;
    while l < n && r < n {
        if set.len() > k || r == n {
            // 減らさないといけないので、lを大きくする
            let size :u64 = map[&a[l]];
            if size == 1 {
                map.remove(&a[l]);
                set.remove(&a[l]);
            } else {
                map.insert(a[l], size - 1);
            }
            l += 1;
        } else {
            set.insert(a[r]);
            *map.entry(a[r]).or_insert(0) += 1;
            r += 1;
        }

        if set.len() <= k {
            ans.chmax(r - l);
        }
    }

    eprintln!("{} {}", l, r);
    println!("{}", ans);
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
        return
            if *self > elm {
                *self = elm;
                true
            } else { false };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return
            if *self < elm {
                *self = elm;
                true
            } else { false };
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
