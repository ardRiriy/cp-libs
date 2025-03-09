use std::{cmp::Reverse, collections::BinaryHeap};

use cps::consts::{DI, DJ, INF};
use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input!{
        h: usize,
        w: usize,
        start: (Usize1, Usize1),
        goal: (Usize1, Usize1),
        s: [Chars; h],
    }


    let mut pq = BinaryHeap::new();
    let mut seen = vec![vec![INF; w]; h];
    pq.push(Reverse((0, start)));

    while let Some(Reverse((c, (pi, pj)))) = pq.pop() {
        if seen[pi][pj] != INF { continue; }
        seen[pi][pj] = c;
        for r in 0..4 {
            let ni = pi.wrapping_add(DI[r]);
            let nj = pj.wrapping_add(DJ[r]);
            if ni < h && nj < w && seen[ni][nj] == INF && s[ni][nj] == '.' {
                pq.push(Reverse((c, (ni, nj))));
            }
        }
        for di in &[!1, !0, 0, 1, 2] {
            for dj in &[!1, !0, 0, 1, 2] {
                let ni = pi.wrapping_add(*di);
                let nj = pj.wrapping_add(*dj);
                if ni < h && nj < w && seen[ni][nj] == INF && s[ni][nj] == '.' {
                    pq.push(Reverse((c+1, (ni, nj))));
                }
            } 
        }
    }
    println!("{}", if seen[goal.0][goal.1] == INF { -1 } else { seen[goal.0][goal.1] as i64 });
}

