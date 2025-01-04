use std::collections::VecDeque;

use cps::consts::{DI, DJ, INF};
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut start=(0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'S' {
                start = (i, j);
            }
            if s[i][j] == 'G' {
                goal = (i, j);
            }
        }
    }

    let mut seen = vec![vec![vec![INF; w]; h]; 2];
    seen[0][start.0][start.1] = 0;
    seen[1][start.0][start.1] = 0;

    let mut que = VecDeque::new();
    que.push_back((0, start));
    que.push_back((1, start));


    while let Some((i, (pi, pj))) = que.pop_front() {
        let nxt= 1 - i;
        for r in 0..4 {
            if r % 2 == i {
                continue;
            }

            let ni = pi.wrapping_add(DI[r]);
            let nj = pj.wrapping_add(DJ[r]);
            if ni < h && nj < w && seen[nxt][ni][nj] == INF && s[ni][nj] != '#' {
                seen[nxt][ni][nj] = seen[i][pi][pj] + 1;
                que.push_back((nxt, (ni, nj)));
            }
        }
    }

    let ans = seen[0][goal.0][goal.1].min(seen[1][goal.0][goal.1]);
    println!("{}", if ans == INF { -1 } else { ans as i64 });
}

