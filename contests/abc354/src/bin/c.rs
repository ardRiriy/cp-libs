use std::{cmp::Reverse, collections::BTreeSet};

use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        mut cd: [(u64, u64); n]
    }

    let mut cards = cd
        .iter()
        .enumerate()
        .map(|(idx, a)| (a.0, a.1, idx))
        .collect_vec();

    cards.sort_by_key(|&(_, c, _)| Reverse(c));

    let mut ans = vec![true; n];
    let mut set = BTreeSet::new();
    for &(a, _) in &cd {
        set.insert(a);
    }

    for &(a, c, idx) in cards.iter() {
        if let Some(x) = set.iter().max() {
            if *x != a {
                ans[idx] = false;
            }
        }
        set.remove(&a);
    }

    println!("{}", ans.iter().filter(|b| **b).count());
    println!(
        "{}",
        ans.iter()
            .enumerate()
            .filter_map(|(idx, &b)| if b { Some(idx + 1) } else { None })
            .join(" ")
    );
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
        } else {
            false
        }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
            *self = elm;
            true
        } else {
            false
        }
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
