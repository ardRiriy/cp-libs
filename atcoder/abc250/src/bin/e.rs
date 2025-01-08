use std::collections::BTreeSet;

#[allow(unused_imports)]
use cps::debug::*;
use cps::zobrist_hash::ZobristHash;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
        b: [u64; n],
        q: usize,
    }

    let mut a_csum = vec![0];
    let mut b_csum = vec![0];
    let mut zh = ZobristHash::new();

    {
        let mut seen = BTreeSet::new();
        for (i, &ai) in a.iter().enumerate() {
            if seen.contains(&ai) {
                let key = a_csum.last().unwrap();
                a_csum.push(*key);
            } else {
                seen.insert(ai);
                let key = a_csum[i] + zh.get(ai);
                a_csum.push(key);
            }
        }
    }
    {
        let mut seen = BTreeSet::new();
        for (i, &bi) in b.iter().enumerate() {
            if seen.contains(&bi) {
                let key = b_csum.last().unwrap();
                b_csum.push(*key);
            } else {
                seen.insert(bi);
                let key = b_csum[i] + zh.get(bi);
                b_csum.push(key);
            }
        }
    }

    for _ in 0..q {
        input! {
            x: usize,
            y: usize,
        }
        println!("{}", if a_csum[x] == b_csum[y] { "Yes" } else { "No" });
    }
}

