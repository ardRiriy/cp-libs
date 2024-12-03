use std::collections::{BTreeMap, BTreeSet};
use cps::multiset::MultiSet;
use proconio::input;

#[allow(dead_code)]
fn solve1(q: usize) {
    // コンテスト本番実装
    let mut cnt = BTreeMap::new();
    let mut set = BTreeSet::new();

    for _ in 0..q {
        input! {
            t: u32,
        }

        match t {
            1 => {
                input! {
                    x: u32,
                }
                set.insert(x);
                *cnt.entry(x).or_insert(0u64) += 1;
            },
            2 => {
                input! {
                    x: u32,
                }
                if cnt[&x] == 1 {
                    cnt.insert(x, 0);
                    set.remove(&x);
                } else {
                    let val = cnt[&x];
                    cnt.insert(x, val - 1);
                }
            }
            3 => {
                println!("{}", set.len());
            },
            _ => { unreachable!(); }
        }
    }
}

fn solve2(q: usize) {
    let mut mset = MultiSet::new();
    for _ in 0..q {
        input! {
            t: u32,
        }
        match t {
            1 => {
                input! {
                    x: u32,
                }
                mset.add(x, 1u64);
            }
            2 => {
                input! {
                    x: u32,
                }
                mset.remove(x, 1);
            },
            3 => {
                println!("{}", mset.len());
            }
            _ => { }
        }
    }
}

fn main() {
    input!{
        q: usize,
    }

    // solve1(q);
    solve2(q);
}
