use std::collections::{BTreeMap, VecDeque};

use cps::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n, |a: &u64, _| { *a } );

    let g = edges.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            uf.merge(u, v);
            g
        });
    

    let inf = 1 << 60;
    let mut color = vec![inf; n];

    for i in 0..n {
        if color[i] != inf {
            continue;
        }

        let mut que = VecDeque::new();
        color[i] = 0;
        que.push_back(i);
        while let Some(p) = que.pop_front() {
            for &ni in &g[p] {
                if color[ni] == color[p] {
                    println!("0");
                    return;
                } 
                if color[ni] == inf {
                    color[ni] = 1 - color[p];
                    que.push_back(ni);
                }
            }
        }
    }

    // dbg!(&color);
    let mut cnt = vec![vec![0; 2]; n];
    for i in 0..n {
        cnt[uf.leader(i)][color[i]] += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        ans += n - uf.size(i) + cnt[uf.leader(i)][1 - color[i]] - g[i].len();
    }
    println!("{}", ans / 2);

}

