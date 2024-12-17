use std::{cmp::Reverse, collections::BinaryHeap};

use cps::consts::{DI, DJ};
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let inf: u64 = 1 << 60;
    let mut seen = vec![vec![inf; w]; h];
    let mut que = BinaryHeap::new();
    que.push(Reverse((0, 0usize, 0usize)));

    while let Some(Reverse((c, pi, pj))) = que.pop() {
        if seen[pi][pj] != inf {
            continue;
        }
        seen[pi][pj] = c;

        for r in 0..4 {
            let ni = pi.wrapping_add(DI[r]);
            let nj = pj.wrapping_add(DJ[r]);
            if ni >= h || nj >= w || seen[ni][nj] != inf {
                continue;
            }

            if s[ni][nj] == '.' {
                que.push(Reverse((c, ni, nj)));
            } else {
                que.push(Reverse((c + 1, ni, nj)));

                for nr in 0..8 {
                    let nni = ni.wrapping_add(DI[nr]);
                    let nnj = nj.wrapping_add(DJ[nr]);
                    if nni >= h || nnj >= w || seen[nni][nnj] != inf {
                        continue;
                    }
                    que.push(Reverse((c + 1, nni, nnj)));
                }
            }
        }
    }
    println!("{}", seen[h - 1][w - 1]);
}
