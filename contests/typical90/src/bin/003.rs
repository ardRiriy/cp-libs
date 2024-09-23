use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn bfs(g: &Vec<Vec<usize>>, start: usize, dist: &mut Vec<u64>) {
    let mut que = VecDeque::new();
    que.push_back(start);
    dist[start] = 0;

    while let Some(pos) = que.pop_front() {
        for &ni in g[pos].iter() {
            if dist[ni] == INF {
                dist[ni] = dist[pos] + 1;
                que.push_back(ni);
            }
        }
    }
}

fn solve() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n-1]
    }

    let g = edges.iter().fold(
        vec![vec![]; n],
        |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            g
        }
    );

    let mut dist = vec![INF; n];
    bfs(&g, 0, &mut dist);
    let (u, _) = dist.iter().enumerate().max_by_key(|(_, &x)| x).unwrap();

    dist.fill(INF);
    bfs(&g, u, &mut dist);

    println!("{}", dist.iter().max().unwrap() + 1);
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
            } else { false }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
                *self = elm;
                true
            } else { false }
    }
}


fn main() {
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
