use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn solve() {
    input! {
        q: usize
    }
    let mut set: BTreeSet<u64> = BTreeSet::new();
    let mut map = BTreeMap::new();
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {

                x: u64
                    }
                *map.entry(x).or_insert(0) += 1u64;
                set.insert(x);
            }
            2 => {
                input! {
                    x: u64,
                    c: u64
                }
                let v = (match map.get(&x) {
                    Some(v) => *v,
                    None => 0,
                } as i64
                    - c as i64)
                    .max(0) as u64;
                map.insert(x, v);
                if v == 0 {
                    set.remove(&x);
                }
            }
            3 => {
                let min = *set.iter().min().unwrap();
                let max = *set.iter().max().unwrap();
                println!("{}", max - min);
            }

            _ => unreachable!(),
        }
    }
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
