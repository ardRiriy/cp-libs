use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn solve() {
    input!{
        n: usize,
        m: usize,
        a: [u64; n],
        edges: [(Usize1, Usize1, u64); m]        
    }

    let g = 
        edges.iter()
        .fold(vec![vec![]; n], |mut acc, &(u, v, w)| {
            acc[u].push((v, w + a[u]));
            acc[v].push((u, w + a[v]));
            acc
        });

    let mut dist = vec![INF; n];

    let mut que = BinaryHeap::new();
    que.push(Reverse((0, 0)));

    while let Some(Reverse((cost, p))) = que.pop() {
        if dist[p] != INF {
            continue;
        }

        dist[p] = cost;

        for &(np, w) in &g[p] {
            if dist[np] == INF {
                que.push(Reverse((cost + w, np)));
            }
        }
    }


    // for (idx, &d) in dist.iter().enumerate() {
    //     if idx == 0 {
    //         continue;        
    //     }
    //     println!("{}", d + a[idx]);
    // }

    println!("{}", dist.iter().enumerate().filter_map(|(idx, x)| if idx != 0 { Some(x + a[idx]) } else { None }).join(" "));
}

/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/

static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        if *self > elm {
            *self = elm;
            true
        } else {
            false
        }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
            *self = elm;
            true
        } else {
            false
        }
    }
}

fn main() {
    // input! { mut i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}

