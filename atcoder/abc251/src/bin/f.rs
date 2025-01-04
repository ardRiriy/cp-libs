use std::collections::VecDeque;

use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }
    let g = e.iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut g, (i, &(u, v))| {
            g[u].push((v, i));
            g[v].push((u, i));
            g
    });

    let inf = 1<<60;

    // t1
    {
        let mut seen = vec![false; n];
        let mut stk = vec![(0, 1<<60)];

        while let Some((p, ei)) = stk.pop() {
            if seen[p] {
                continue;
            }
            seen[p] = true;
            if ei != inf {
                println!("{} {}", e[ei].0+1, e[ei].1+1);
            }

            for &(ni, ei) in g[p].iter() {
                if seen[ni] {
                    continue;
                }
                stk.push((ni, ei));
            }
        }
    }

    // t2
    {
        let mut seen = vec![false; n];
        let mut que = VecDeque::new();
        que.push_back((0, inf));

        while let Some((p, ei)) = que.pop_front() {
            if seen[p] {
                continue;
            }
            seen[p] = true;
            if ei != inf {
                println!("{} {}", e[ei].0+1, e[ei].1+1);
            }

            for &(ni, ei) in g[p].iter() {
                if seen[ni] {
                    continue;
                }
                que.push_back((ni, ei));
            }
        }
    }
}

