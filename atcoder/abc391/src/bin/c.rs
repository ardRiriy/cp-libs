use std::collections::BTreeSet;
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        q: usize,
    }
    let mut set = BTreeSet::new();
    let mut count = vec![1; n];
    let mut pos = (0..n).collect_vec();

    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                p: Usize1,
                h: Usize1
            }
            let before = pos[p];
            count[before] -= 1;
            if count[before] == 1 {
                set.remove(&before);
            }

            pos[p] = h;
            count[h] += 1;
            if count[h] == 2 {
                set.insert(h);
            }

        } else {
            println!("{}", set.len());
        }
    }
}

