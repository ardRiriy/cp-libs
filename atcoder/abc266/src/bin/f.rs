use std::collections::BTreeSet;

use ac_library::Dsu;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        e: [(Usize1, Usize1); n],
        q: usize,
    }
    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)| {
            g[u].push(v);
            g[v].push(u);
            g
        });
    
    static INF :usize = 1<<31;
    let mut prevs = vec![INF; n];
    let mut stk = vec![(0, n)];
    let mut set = BTreeSet::new();
    'dfs: while let Some((p, from)) = stk.pop() {
        prevs[p] = from;
        for &ni in g[p].iter() {
            if prevs[p] == ni {
                continue;
            }
            if prevs[ni] != INF {
                // 閉路発見
                let mut cur = p;
                while cur != ni {
                    set.insert(cur);
                    cur = prevs[cur];
                }
                set.insert(cur);
                break 'dfs; // 閉路は一つだけ
            }
            stk.push((ni, p));
        }
    }

    let mut seen = vec![false; n];

    let mut uf = Dsu::new(n);
    for i in 0..n {
        if seen[i] {
            continue;
        }
        if !set.contains(&i) {
            continue;
        }

        seen[i] = true;
        let mut stk = vec![i];
        while let Some(p) = stk.pop() {
            for &ni in g[p].iter() {
                if seen[ni] || set.contains(&ni) {
                    continue;
                }
                uf.merge(p, ni);
                stk.push(ni);
                seen[ni] = true;
            }
        }
    }

    for _ in 0..q {
        input! {
            u: Usize1,
            v: Usize1,
        }
        println!("{}", if uf.same(u, v) { "Yes" } else { "No" });
    }
}

