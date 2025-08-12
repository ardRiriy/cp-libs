use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap}};

use library::utils::{consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let (n, m, k) = ip.triple::<usize, usize, usize>();
    let a = ip.vector::<usize>(k).iter().map(|i| *i-1).collect::<Vec<_>>();
    let g = ip.weighted_graph::<u64>(n, m, false, true);
    
    let mut livehouse = BTreeSet::from_iter(a.iter().copied());
    livehouse.remove(&0);
    livehouse.remove(&(n-1));

    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0, 0, !0));

    // seen[i][j][k] := 都市i到達時点で(都市0, n-1を除いて)j回ライブをしており、その時の移動距離の小さい方からk番目
    let mut seen = vec![vec![vec![INF; 2]; 3]; n];
    let mut from = vec![vec![vec![INF as usize; 2]; 3]; n];
    while let Some((Reverse(c), p, q, adr)) = que.pop() {
        if seen[p][q][1] != INF {
            continue;
        }
        
        if seen[p][q][0] == INF {
            seen[p][q][0] = c;
            from[p][q][0] = adr;
        } else {
            if from[p][q][0] == adr {
                continue;
            }
            seen[p][q][1] = c; 
            from[p][q][1] = adr;
        }
        
        if livehouse.contains(&p) && adr != p && q+1 <= 2 && seen[p][q+1][1] == INF {
            if q==1 {
            }
            que.push((Reverse(c), p, q+1, p));
        }

        for &(ni, w) in g[p].iter() {
            // liveしない
            if  seen[ni][q][1] == INF {
                que.push((Reverse(c+w), ni, q, adr));
            }
        }
    }
    println!("{}", seen[n-1][2][0]);
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
