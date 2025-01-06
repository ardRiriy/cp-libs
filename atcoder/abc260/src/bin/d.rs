use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        k: usize,
        p: [Usize1; n],
    }

    let mut ans = vec![-1; n];
    let inf = 1usize << 60;

    let mut parent = vec![inf; n];
    let mut dist = vec![0; n];

    let mut tops = BTreeSet::new();

    for (t, &pi) in p.iter().enumerate() {
        if let Some(&pos) = tops.range(pi..).next() {
            parent[pi] = pos;
            dist[pi] = dist[pos] + 1;
            tops.remove(&pos);
            tops.insert(pi);
        } else {
            tops.insert(pi);
            parent[pi] = !0;
            dist[pi] = 1;
        }
        if dist[pi] == k {
            let mut i = pi;
            while i != !0 {
                ans[i] = t as i64 + 1;
                i = parent[i];
            }
            tops.remove(&pi);
        }
    }
    println!("{}", ans.iter().join("\n"));
}

