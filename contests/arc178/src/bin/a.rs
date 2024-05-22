use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; m]
    }
    a.sort();
    if a[0] == 1 || a[m - 1] as usize == n {
        println!("-1");
        return;
    }

    let mut horyu = INF;
    let mut ans = vec![INF; n];
    let mut ans_idx = 1;
    ans[0] = 1;

    let mut set = BTreeSet::new();
    let mut que = VecDeque::from((2..=n as u64).into_iter().collect_vec());
    for &x in &a {
        set.insert(x);
    }

    while let Some(x) = que.pop_front() {
        if set.contains(&x) {
            if horyu == INF && ans_idx + 1 == x as usize {
                horyu = x;
            } else {
                ans[ans_idx] = x;
                ans_idx += 1;
            }
        } else {
            ans[ans_idx] = x;
            ans_idx += 1;

            if horyu != INF {
                que.push_front(horyu);
                horyu = INF;
            }
        }
    }
    println!("{}", ans.iter().join(" "));
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
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
