use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input};
fn solve() {
    input!{
        h: usize,
        w: usize,
        y: u64,
        a: [[u64; w]; h]
    }

    let mut seen = vec![vec![false; w]; h];
    let mut pq: BinaryHeap<(Reverse<u64>, usize, usize)> = BinaryHeap::new();
    for i in 0..h {
        for j in 0..w {
            if (i == 0 || i == h-1) || (j == 0 || j == w-1) {
                seen[i][j] = true;
                pq.push((Reverse(a[i][j]), i, j));
            } 
        }
    }

    let di = &[!0, 0, 1, 0];
    let dj = &[0, !0, 0, 1];

    let mut current = h * w;
    for yi in 1..=y {
        while let Some((Reverse(height), pi, pj)) = pq.pop() {
            if height > yi {
                pq.push((Reverse(height), pi, pj));
                break;
            }
            current -= 1;

            for r in 0..4 {
                let ni = pi.wrapping_add(di[r]);
                let nj = pj.wrapping_add(dj[r]);

                if ni < h && nj < w && !seen[ni][nj] {
                    seen[ni][nj] = true;
                    pq.push((Reverse(a[ni][nj]), ni, nj));
                }
            }

        }

        println!("{current}");
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

