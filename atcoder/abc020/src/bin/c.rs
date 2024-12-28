use std::{cmp::Reverse, collections::BinaryHeap};

use cps::consts::{DI, DJ, INF};
#[allow(unused_imports)]
use cps::debug::*;
use itertools::iproduct;
use proconio::{input, marker::Chars};

fn bfs(start: (usize, usize), t: u64, board: &Vec<Vec<char>>, limit: u64) -> bool {
    let h = board.len();
    let w = board[0].len();

    let mut seen = vec![vec![INF; w]; h];
    let mut pq = BinaryHeap::from([(Reverse(0), start)]);

    while let Some((Reverse(c), (pi, pj))) = pq.pop() {
        if seen[pi][pj] != INF {
            continue;
        }
        seen[pi][pj] = c;

        if board[pi][pj] == 'G' {
            return c <= limit;
        }

        for r in 0..4 {
            let ni = pi.wrapping_add(DI[r]);
            let nj = pj.wrapping_add(DJ[r]);

            if ni < h && nj < w && seen[ni][nj] == INF {
                if board[ni][nj] == '#' {
                    pq.push((Reverse(c+t), (ni, nj)));
                } else {
                    pq.push((Reverse(c+1), (ni, nj)));
                }
            }
        }
    }

    false
}

fn main() {
    input!{
        h: usize,
        w: usize,
        t: u64,
        ss: [Chars; h],
    }

    let mut start = (0, 0);
    'outer: for (i, j) in iproduct!(0..h, 0..w) {
        if ss[i][j] == 'S' {
            start = (i, j);
            break 'outer;
        }
    }

    let mut ok = 1;
    let mut ng = INF;

    while ng - ok > 1 {
        let mid = (ng+ok)/2;
        if bfs(start, mid, &ss, t) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}


