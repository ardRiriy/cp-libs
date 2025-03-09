use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],        
    }
    let mut ans = 0;

    let mut g = vec![BTreeSet::new(); n];
    for &(u, v) in e.iter() {
        if u == v || g[u].contains(&v) {
            ans += 1;
        }
        g[u].insert(v);
        g[v].insert(u);
    }

    println!("{}", ans);
}

