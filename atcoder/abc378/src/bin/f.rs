use std::collections::VecDeque;
use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1); n-1],
    }

    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| { g[u].push(v); g[v].push(u); g });

    let mut uf = Dsu::new(n);
    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] || g[i].len() != 3 {
            continue;
        }

        let mut que = VecDeque::new();
        que.push_back(i);
        seen[i] = true;

        while let Some(p) = que.pop_front() {
            for &ni in g[p].iter() {
                if !seen[ni] && g[ni].len() == 3 {
                    seen[ni] = true;
                    uf.merge(i, ni);
                    que.push_back(ni);
                }
            }
        }
    }

    fn sum(n: u64) -> u64 {
        n * (n - 1) / 2
    } 

    let mut cnt = vec![0; n];
    for i in 0..n {
        if !seen[i] {
            continue;
        }

        for &ni in g[i].iter() {
            if g[ni].len() == 2 {
                cnt[uf.leader(i)] += 1;
            }
        }
    }
    println!("{}", cnt.iter().map(|&x| if x == 0 { 0 } else { sum(x) }).sum::<u64>());    
    
}