use std::collections::VecDeque;

use cps::consts::INF;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        c: [Chars; n],
    }

    let mut seen = vec![vec![INF; n]; n];
    let mut que = VecDeque::new();
    for i in 0..n {
        seen[i][i] = 0;
        que.push_back((i, i));
    }
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            if c[i][j] != '-' {
                seen[i][j] = 1;
                que.push_back((i, j));
            }
        }
    }

    while let Some((pi, pj)) = que.pop_front() {
        for ni in 0..n {
            for nj in 0..n {
                if c[ni][pi] == c[pj][nj] && c[ni][pi] != '-' && seen[ni][nj] == INF {
                    seen[ni][nj] = seen[pi][pj] + 2;
                    que.push_back((ni, nj));
                }
            }
        }
    }
    println!("{}", seen.iter().map(|v| v.iter().map(|vi| if vi == &INF { -1 } else { *vi as i64 }).join(" ")).join("\n"));

}

