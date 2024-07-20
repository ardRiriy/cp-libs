use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};
fn solve() {
    input!{
        n: usize,
        k: usize,
        mut s: Chars
    }
    
    let mut sv = vec![];

    let mut ans = 0;
    'l: for v in s.iter().permutations(n).sorted().dedup() {
        // println!("{:?}", v);
        sv = v[0..k].iter().map(|c| **c).collect_vec();
        if sv.iter().eq(sv.iter().rev()) {
            continue 'l;
        }

        for i in k..n {
            // vの0からi番目までの文字列をsvに追加
            sv.push(*v[i]);
            sv.remove(0);
            
            // rintln!("{:?}", sv);
            // svが回分ならcontinue
            if sv.iter().eq(sv.iter().rev()) {
                continue 'l;
            }
        }

        ans += 1;
    }
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

