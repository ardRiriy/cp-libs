use std::{cmp::Reverse, collections::BinaryHeap};

use cps::consts::{DI, DJ};
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        h: usize,
        w: usize,
        x: u128,
        p: Usize1,
        q: Usize1,
        v: [[u128; w]; h],
    }

    let mut strng = v[p][q];

    let mut seen = vec![vec![false; w]; h];
    seen[p][q] = true;
    let mut pq = BinaryHeap::new();
    for r in 0..4 {
        let ni = p.wrapping_add(DI[r]);
        let nj = q.wrapping_add(DJ[r]);
        if ni < h && nj < w {
            seen[ni][nj] = true;
            pq.push((Reverse(v[ni][nj]), ni, nj));
        }
    }

    while let Some((Reverse(c), i, j)) = pq.pop() {
        if strng <= c * x {
            continue;
        }
        strng += c;
        for r in 0..4 {
            let ni = i.wrapping_add(DI[r]);
            let nj = j.wrapping_add(DJ[r]);
            if ni < h && nj < w && !seen[ni][nj] {
                seen[ni][nj] = true;
                pq.push((Reverse(v[ni][nj]), ni, nj));
            }
        }
    }

    println!("{}", strng);
}

