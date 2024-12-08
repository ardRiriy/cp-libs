use std::collections::VecDeque;

use cps::consts::{DI, DJ};
#[allow(unused_imports)]
use cps::debug::*;
use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        h: usize,
        w: usize,
        d: u64,
        s: [Chars; h],
    }

    let inf = 1u64 << 60;
    let mut seen = vec![vec![inf; w]; h];
    let mut que = VecDeque::new();
    for (i, j) in iproduct!(0..h, 0..w) {
        if s[i][j] == 'H' {
            que.push_back((i, j));
            seen[i][j] = 0;
        }
    }

    while let Some((pi, pj)) = que.pop_front() {
        for r in 0..4 {
            let ni = pi.wrapping_add(DI[r]);
            let nj = pj.wrapping_add(DJ[r]);
            if ni < h && nj < w && s[ni][nj] == '.' && seen[ni][nj] == inf {
                seen[ni][nj] = seen[pi][pj] + 1;
                que.push_back((ni, nj));
            } 
        }
    }
    println!("{}", seen.iter().flatten().filter(|&cnt| *cnt <= d).count());

}

