use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque}};

use itertools::Itertools;
use proconio::{input, marker::{Chars, Usize1}};

fn solve() {
    input! {
        h: usize,
        w: usize,
        (si, sj): (Usize1, Usize1),
        (gi, gj): (Usize1, Usize1),
        s: [Chars; h]
    }

    let di = &[0, 1, 0, !0];
    let dj = &[1, 0, !0, 0];

    let mut pq = VecDeque::new();
    let mut seen = vec![vec![(INF, 0u8); w]; h];

    for r in 0.. 4 {
        pq.push_back((0, r, si, sj));
    }
    
    while let Some((cost, r, pi, pj)) = pq.pop_front() {
        if seen[pi][pj].1 >> r & 1 == 1 {
            continue;
        }
        seen[pi][pj].1 |= 1 << r;

        let mut ni = pi.wrapping_add(di[r]);
        let mut nj = pj.wrapping_add(dj[r]);
        while ni < h && nj < w && s[ni][nj] == '.' {
            if seen[ni][nj].1 >> r & 1 == 1 {
                break;
            }~

            if seen[ni][nj].0 == INF {
                seen[ni][nj].1 |= 1 << r;
                seen[ni][nj].1 |= 1 << ((r + 2) % 4);~~
                pq.push_back((cost + 1, (r + 1) % 4, ni, nj));
                pq.push_back((cost + 1, (r + 3) % 4, ni, nj));
                seen[ni][nj].0 = cost;
            }
            ni = ni.wrapping_add(di[r]);
            nj = nj.wrapping_add(dj[r]);
        }

        // println!("{}", seen.iter().map(|v| v.iter().map(|x| if *x == INF { format!("__") } else { format!("{:2}", x) }).join(" ")).join("\n"));
    }

    println!("{}", seen[gi][gj].0);

}

/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/


static INF: u32 = 1e9 as u32;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        return
            if *self > elm {
                *self = elm;
                true
            } else { false };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return
            if *self < elm {
                *self = elm;
                true
            } else { false };
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
