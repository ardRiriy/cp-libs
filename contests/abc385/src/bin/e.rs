#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};


fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut g = vec![vec![]; n];
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut deg = vec![0; n];
    for i in 0..n {
        deg[i] = g[i].len();
    }
    let mut m_size = 1; 
    for v in 0..n {
        let mut lc: Vec<usize> = g[v]
            .iter()
            .map(|&u| deg[u] - 1)
            .collect();

        lc.sort_unstable_by(|a, b| b.cmp(a));

        let d = lc.len();
        let mut tmp = 1;
        for k in 1..=d {
            let y = lc[k - 1]; 
            let size = 1 + k * (1 + y);
            if size > tmp {
                tmp = size;
            }
        }

        if tmp > m_size {
            m_size = tmp;
        }
    }

    let ans = n - m_size;
    println!("{}", ans);
}


