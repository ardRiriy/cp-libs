use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }
    let mut r = e.iter()
        .fold(vec![0; n], |mut v, &(_, u)| {
            v[u] += 1;
            v
        });

    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });

    let mut ans = vec![];
    let mut heap = BinaryHeap::from_iter((0..n).filter(|i| r[*i] == 0).map(|i| Reverse(i)));
    while let Some(Reverse(p)) = heap.pop() {
        ans.push(p+1);
        for ni in g[p].iter() {
            r[*ni] -= 1;
            if r[*ni] == 0 {
                heap.push(Reverse(*ni));
            }
        }
    }
    println!("{}", if ans.len() == n { ans.iter().join(" ") } else { "-1".to_string() });
}

