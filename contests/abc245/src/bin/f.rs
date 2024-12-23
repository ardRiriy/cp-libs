use std::collections::VecDeque;

#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }

    let mut deg = vec![0; n];
    let g = e.iter().fold(vec![vec![]; n], |mut g, &(a, b)| {
        g[b].push(a);
        deg[a] += 1;
        g
    });

    let mut check = vec![false; n];
    let mut que = VecDeque::new();

    for i in 0..n {
        if deg[i] == 0 {
            que.push_back(i);
        }
    }

    while let Some(v) = que.pop_front() {
        check[v] = true;
        for &ni in &g[v] {
            deg[ni] -= 1;
            if deg[ni] == 0 {
                que.push_back(ni);
            }
        }
    }

    println!("{}", check.iter().filter(|&&x| !x).count());

}
