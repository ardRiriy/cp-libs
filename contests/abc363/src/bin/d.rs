use std::fmt::format;

use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn solve() {
    input!{
        n: usize,
    }

    let mut ng = !0u128;
    let mut ok = 90000000000000000000000000000000011u128;

    while ok.wrapping_sub(ng) > 1 {
        let mid = ok.wrapping_add(ng) / 2;
        let cnt = count(mid);

        if cnt < n as u128 {
            ng = mid;
        } else {
            ok = mid;
        }
    }

    println!("{}", ok);
}

fn count(n: u128) -> u128 {
    let mut res = 0;
    
    for i in 0..=9 {
        if i <= n {
            res += 1;
        } else {
            return res;
        }
    }
    // S = str(X)rev(X)について、Xとしてあり得るものを二分探索で求める
    let mut ok = 1u128;
    let mut ng = 1u128 << 61;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let s = format!("{}{}", mid, mid.to_string().chars().rev().collect::<String>());
        let s_num = s.parse::<u128>().unwrap();

        if s_num <= n {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    res += ok;
    // S = str(X)(i)rev(X)について、Xとしてあり得るものを二分探索で求める
    for i in 0..=9 {
        let mut ok = 1u128;
        let mut ng = 1u128 << 61;

        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let s = format!("{}{}{}", mid, i ,mid.to_string().chars().rev().collect::<String>());
            let s_num = s.parse::<u128>().unwrap();
    
            if s_num <= n {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        res += ok;
    }
    res
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

