use std::collections::VecDeque;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m]
    }
    let g = e.iter()
            .fold(vec![vec![]; n], |mut g, &(u, v)| {
                g[u].push(v);
                g
            });

    let inf = 1u64 << 60;
    let mut dist = vec![inf; n];
    dist[0] = 0;
    
    let mut que = VecDeque::new();
    que.push_back(0);

    while let Some(p) = que.pop_front() {
        for &ni in g[p].iter() {
            if ni == 0 {
                println!("{}", dist[p] + 1);
                return;
            }
            if dist[ni] == inf {
                dist[ni] = dist[p] + 1;
                que.push_back(ni);
            }
        }
    }
    println!("-1");
}

