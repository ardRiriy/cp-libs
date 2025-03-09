

use std::collections::VecDeque;

use proconio::{input, marker::{Chars, Usize1}};

static DI: &[usize] = &[!0, 1, 1, !0];
static DJ: &[usize] = &[!0, !0, 1, 1];
static INF: u64 = 1<<60;

fn main() {
    input!{
        n: usize,
        s: (Usize1, Usize1),
        t: (Usize1, Usize1),
        c: [Chars; n],
    }
    let mut seen = vec![vec![vec![INF; n]; n]; 2];

    seen[0][s.0][s.1] = 0;
    let mut pq = VecDeque::from([(0, s, true)]);
    while let Some((i, (pi, pj), flag)) = pq.pop_front() {
        // 裏が未達ならそれをいれる
        if seen[i^1][pi][pj] == INF {
            seen[i^1][pi][pj] = seen[i][pi][pj];
            pq.push_front((i^1, (pi, pj), true));
        }

        if !flag { continue; }
        for r in 0..4 {
            if r & 1 != i { continue; }
            let mut ni = pi.wrapping_add(DI[r]);
            let mut nj = pj.wrapping_add(DJ[r]);
            while ni<n && nj<n {
                if c[ni][nj] == '#' { break; }
                if seen[i][ni][nj] == INF { 
                    seen[i][ni][nj] = seen[i][pi][pj] + 1;
                    pq.push_back((i, (ni, nj), false));
                }
                ni = ni.wrapping_add(DI[r]);
                nj = nj.wrapping_add(DJ[r]);
            }
        }
    }

    println!("{}", if seen[0][t.0][t.1] == INF && seen[1][t.0][t.1] == INF { -1 } else { seen[0][t.0][t.1].min(seen[1][t.0][t.1]) as i64 });
}


