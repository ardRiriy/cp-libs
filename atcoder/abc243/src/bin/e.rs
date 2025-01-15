use cps::warshall_floyd::{DefaultWFelm, WarshallFloyd};
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1, u64); m],
    }

    let mut g = vec![vec![]; n];

    for &(u, v, w) in e.iter() {
        g[u].push((v, w));
        g[v].push((u, w));
    }

    let wf :WarshallFloyd<u64, DefaultWFelm> = WarshallFloyd::new(&g, DefaultWFelm);

    let mut ans = 0;

    'e: for &(u, v, w) in e.iter() {
        if wf.get(u, v) < w {
            ans += 1;
            continue;
        }
        for i in 0..n {
            if i == u || i == v {
                continue;
            }
            if wf.get(u, i) + wf.get(i, v) <= w{
                // ある頂点iを通って、コストwより小さく移動できるのであれば除去可能
                ans += 1;
                continue 'e;
            }
        }
    }
    println!("{}", ans);
}

