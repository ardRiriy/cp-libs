use std::collections::VecDeque;

use proconio::{input, marker::Chars};
fn solve() {
    input!{
        h: usize,
        w: usize,
        c: [Chars; h]
    }

    let mut seen = vec![vec![INF; w]; h];
    let mut que = VecDeque::new();

    que.push_back((0, 0));
    seen[0][0] = 1;

    let di: &[usize] = &[1, 0, !0, 0];
    let dj: &[usize]  = &[0, 1, 0, !0];

    while let Some((pi, pj)) = que.pop_front() {
         for r in 0..2 {
            let ni = pi + di[r];
            let nj = pj + dj[r];

            if ni < h && nj < w && c[ni][nj] == '.' && seen[ni][nj] == INF {
                seen[ni][nj] = seen[pi][pj] + 1;
                que.push_back((ni, nj));
            }
         }    
    }

    println!("{}", seen
        .iter()
        .flatten()
        .filter_map(|&x| if x != INF { Some(x) } else { None })
        .max()
        .unwrap()
    );
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

