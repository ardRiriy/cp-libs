use std::{collections::VecDeque};

use adry_library::consts::{DI, DJ, INF};
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        h: usize,
        w: usize,
        k: usize,
        rc: [(Usize1, Usize1); k],
    }

    let mut seen = vec![vec![INF; w]; h];

    let mut que = VecDeque::new();
    for &(r, c) in rc.iter() {
        que.push_back((r,c,0));
    } 
    
    while let Some((pi,pj, c)) = que.pop_front() {
        if seen[pi][pj] != INF { continue; }
        seen[pi][pj] = c;

        for r in 0..4 {
            let ni = pi.wrapping_add(DI[r]);
            let nj = pj.wrapping_add(DJ[r]);

            if ni >= h || nj >= w || seen[ni][nj] != INF {
                continue;
            }

            let mut v = vec![];
            for rr in 0..4 {
                let nni = ni.wrapping_add(DI[rr]);
                let nnj = nj.wrapping_add(DJ[rr]);
                
                if nni >= h || nnj >= w || seen[nni][nnj] == INF {
                    continue;
                }
                v.push(seen[nni][nnj]);                
            }

            if v.len() >= 2 {
                v.sort();
                que.push_back((ni, nj, v[1]+1));
            }
        }
    }

    let ans = seen.iter().flatten().map(|x| if *x==INF { 0 } else { *x }).sum::<u64>();
    println!("{}", ans);
    
}



// ===== bundled adry_library =====

pub mod adry_library {
    pub mod consts {
        pub static INF: u64 = 1e18 as u64;
        pub static DI: &[usize] = &[0, !0, 0, 1, !0, 1, !0, 1];
        pub static DJ: &[usize] = &[!0, 0, 1, 0, !0, !0, 1, 1];
        pub static DC: &[char] = &['L', 'U', 'R', 'D'];
    }
}

