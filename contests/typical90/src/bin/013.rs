use cps::graph::dijkstra;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, u64); m]
    }

    let g = edges.iter()
        .fold(vec![vec![]; n], |mut acc, (u, v, w)| {
            acc[*u].push((*v, *w));
            acc[*v].push((*u, *w));
            acc
        });

    let dist_from_zero = dijkstra(0, &g);
    let dist_from_n = dijkstra(n-1, &g);

    println!("{}", (0..n).map(|idx| dist_from_zero[idx] + dist_from_n[idx]).join("\n"));
}
