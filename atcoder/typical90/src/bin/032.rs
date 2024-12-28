use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn solve() {
    input! {
        n: usize,
        a: [[i64; n]; n],
        m: usize,
        bad: [(Usize1, Usize1); m]
    }

    let set = bad.iter().fold(BTreeSet::new(),
        |mut acc, &c| {
            acc.insert(c);
            acc
        }
    );

    let mut ans = INF as i64;
    for v in (0..n).permutations(n) {
        if v.windows(2).any(|c| {
            set.contains(&(c[0], c[1])) || set.contains(&(c[1], c[0]))
        }) {
            continue;
        }

        // println!("{:?}", v);
        let sum = v.iter()
            .enumerate()
            .map(|(idx, &i)| a[i][idx])
            .sum::<i64>();
        // println!("{sum}");
        ans.chmin(sum);
    }

    if ans == INF as i64 {
        ans = -1;
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
