use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

#[derive(Debug, Clone, Copy)]
struct Edge {
    to: usize,
    t: u64,
    k: u64,
}

impl Edge {
    fn new(to: usize, t: u64, k: u64) -> Self {
        Self { to, t, k }
    }
    
    fn next(self, t: u64) -> u64 {
        let r = t % self.k;
        if r == 0 {
            t + self.t
        } else {
            t + self.t + self.k - r
        }
    }
}

fn main() {
    input!{
        n: usize,
        m: usize,
        from: Usize1,
        to: Usize1,
        e: [(Usize1, Usize1, u64, u64); m], 
    }
    
    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(from, to, t, k)| {
            g[from].push(Edge::new(to, t, k));
            g[to].push(Edge::new(from, t, k));
            g
        });

    let inf = 1 << 60;
    let mut dist = vec![inf; n];
    
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), from));
    
    while let Some((Reverse(t), p)) = pq.pop() {
        if dist[p] != inf {
            continue;
        }

        dist[p] = t;
        for &e in &g[p] {
            if dist[e.to] == inf {
                let nt = e.next(t);
                pq.push((Reverse(nt), e.to));
            }    
        }
    }
    println!("{}", if dist[to] == inf { -1 } else { i64::try_from(dist[to]).unwrap() });
}

