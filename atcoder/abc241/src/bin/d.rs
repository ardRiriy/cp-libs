use std::collections::BTreeMap;

use proconio::{input};
fn solve() {
    input!{
        q: usize,
    }

    let mut map = BTreeMap::new();

    for _ in 0..q {
        input! {
            t: u8,
            x: u64,
        }

        match t {
            1 => {
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    k: u32
                }
                let mut cnt = 0;
                let mut x = x;
                while cnt < k {
                    match map.range(..=x).next_back() {
                        Some((&key, &val)) => {
                            cnt += val;
                            if cnt >= k {
                                println!("{}", key);
                            } else {
                                x = key - 1;
                            }
                        },
                        None => {
                            println!("-1");
                            break;
                        }
                    }
                }
            }
            3 => {
                input! {
                    k: u32
                }
                let mut cnt = 0;
                let mut x = x;
                while cnt < k {
                    match map.range(x..).next() {
                        Some((&key, &val)) => {
                            cnt += val;
                            if cnt >= k{
                                println!("{}", key);
                            } else {
                                x = key + 1;
                            }
                        },
                        None => {
                            println!("-1");
                            break;
                        }
                    }   
                }
            }
            _ => { panic!() }
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
    // input! { mut i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}

