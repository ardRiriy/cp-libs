use ac_library::Dsu;
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let inf = 1 << 60;
    let mut x = vec![inf; n];
    let mut uf = Dsu::new(n);

    let mut ans = vec![];
    let mut g = vec![vec![]; n];

    for i in 0..q {
        input! {
            a: Usize1,
            b: Usize1,
            d: i64,
        }

        if uf.same(a, b) {
            if x[a] - x[b] == d {
                ans.push(i+1);
            }
        } else {
            if x[a] == inf && x[b] == inf {
                x[b] = 0;
                x[a] = x[b] + d;
            } else if x[a] == inf {
                x[a] = x[b] + d;
            } else if x[b] == inf {
                x[b] = x[a] - d;
            } else {
                if uf.size(a) > uf.size(b) {
                    x[b] = x[a] - d;
                    let mut seen = vec![false; n];
                    let mut stk = vec![b];
                    seen[b] = true;
                    while let Some(p) = stk.pop() {
                        for &(ni, nd) in g[p].iter() {
                            if seen[ni] { continue; }
                            x[ni] = x[p] + nd;
                            stk.push(ni);
                            seen[ni] = true;
                        }
                    }
                } else {
                    x[a] = x[b] + d;
                    let mut seen = vec![false; n];
                    let mut stk = vec![a];
                    seen[a] = true;
                    while let Some(p) = stk.pop() {
                        for &(ni, nd) in g[p].iter() {
                            if seen[ni] { continue; }
                            x[ni] = x[p] + nd;
                            stk.push(ni);
                            seen[ni] = true;
                        }
                    }
                }
            }
            ans.push(i+1);
            uf.merge(a, b);

            g[a].push((b, -d));
            g[b].push((a, d));
        }
    }
    println!("{}", ans.iter().join(" "));
}

