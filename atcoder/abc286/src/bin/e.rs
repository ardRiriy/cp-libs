use cps::consts::INF;
use cps::warshall_floyd::{WFelm, WarshallFloyd};
use proconio::{input, marker::{Chars, Usize1}};

struct Elm;

impl WFelm<(u64, u64)> for Elm {
    fn min(&self, a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
        if a.0 < b.0 {
            a
        } else if a.0 > b.0 {
            b
        } else {
            if a.1 > b.1 {
                a
            } else {
                b
            }
        }
    }

    fn add(&self, a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
        (a.0 + b.0, a.1 + b.1)
    }

    fn infinity(&self) -> (u64, u64) {
        (INF, 0)
    }

    fn identity(&self) -> (u64, u64) {
        (0, 0)
    }
}

fn main() {
    input!{
        n: usize,
        a: [u64; n],
        s: [Chars; n],
        q: usize,
    }
    
    let mut g = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                g[i].push((j, (1, a[j])));
            }
        }
    }
    
    let wf = WarshallFloyd::new(&g, Elm);
    for _ in 0..q {
        input! {
            f: Usize1,
            t: Usize1,
        }
        let ret = wf.get(f, t);
        if ret.0 == INF {
            println!("Impossible");
        } else {
            println!("{} {}", ret.0, ret.1 + a[f]);
        }
    }
}
