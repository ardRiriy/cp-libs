use cps::{chlibs::ChLibs, consts::INF};
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
        edges: [(Usize1, Usize1, u64); m],
        e: [Usize1; k]
    }

    let mut dist = vec![INF; n];
    dist[0] = 0;

    for &idx in &e {
        let (u, v, cost) = edges[idx];
        if dist[u] != INF {
            let next = dist[u] + cost;
            dist[v].chmin(next);
        }
    }

    println!("{}", if dist[n-1] == INF { "-1".to_string() } else { format!("{}", dist[n-1]) });
}