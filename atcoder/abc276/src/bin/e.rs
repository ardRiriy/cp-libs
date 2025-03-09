use std::collections::VecDeque;

use cps::consts::{DI, DJ};
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let (si, sj) = {
        let (p, _) = s.iter()
            .flatten()
            .enumerate()
            .find(|(_, c)| c == &&'S')
            .unwrap();
        (p/w, p%w)
    };

    for (r1, r2) in (0..4).tuple_combinations() {
        let mut seen = vec![vec![false; w]; h];
        
        let gi = si.wrapping_add(DI[r2]);
        let gj = sj.wrapping_add(DJ[r2]);

        let si = si.wrapping_add(DI[r1]);
        let sj = sj.wrapping_add(DJ[r1]);
        if si >= h || sj >= w || s[si][sj] == '#' {
            continue;
        }
        let mut que = VecDeque::new();
        que.push_back((si, sj));
        seen[si][sj] = true;
        while let Some((pi, pj)) = que.pop_front() {
            if (pi, pj) == (gi, gj) {
                println!("Yes");
                return;
            }
            for r in 0..4 {
                let ni = pi.wrapping_add(DI[r]);
                let nj = pj.wrapping_add(DJ[r]);
                if ni < h && nj < w && !seen[ni][nj] && s[ni][nj] == '.' {
                    seen[ni][nj] = true;
                    que.push_back((ni, nj));
                }
            }
        }
    }
    println!("No");
}

