use std::collections::VecDeque;

use library::utils::{consts::{DI, DJ, INF}, input::Input, iterlibs::collect::CollectIter};

fn solve(ip: &mut Input) {
    let (h, w) = ip.pair();
    let s = (0..h).map(|_| ip.chars()).collect_vec();
    
    let mut que = VecDeque::new();
    let mut seen = vec![vec![None; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                que.push_back((i,j));
                seen[i][j] = Some(0);
            }
        }
    }

    while let Some((pi, pj)) = que.pop_front() {
        let d = seen[pi][pj].unwrap();
        for r in 0..4 {
            let ni = pi.wrapping_add(DI[r]);
            let nj = pj.wrapping_add(DJ[r]);
            if ni >= h || nj >= w || seen[ni][nj].is_some() {
                continue;
            }
            
            let cnt = (0..4).filter(|&r| {
                let ni = ni.wrapping_add(DI[r]);
                let nj = nj.wrapping_add(DJ[r]);
                ni < h && nj < w && (seen[ni][nj].is_some() && seen[ni][nj].unwrap() <= d)
            }).count();
            if cnt == 1 {
                seen[ni][nj] = Some(d+1);
                que.push_back((ni,nj));
            } else if cnt >= 2 {
                seen[ni][nj] = Some(INF);
            }
        }
    }

    println!("{}", seen.iter().flatten().filter(|v| v.is_some() && v.unwrap() != INF).count());
}

fn main() {
    static IS_MULTI_TEST_CASE :bool = false;
    let mut ip = Input::new();

    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for _ in 0..t {
        solve(&mut ip);
    }
}
