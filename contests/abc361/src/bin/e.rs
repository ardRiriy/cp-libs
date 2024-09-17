use itertools::Itertools;
use proconio::{input, marker::Usize1};
use cps::graph::dijkstra;
fn main() {
    input!{
        n: usize,
        edges: [(Usize1, Usize1, u64); n-1],
    }

    let g = edges.iter()
        .fold(vec![vec![]; n], |mut acc, (u, v, w)| {
        acc[*u].push((*v, *w));
        acc[*v].push((*u, *w));
        acc
    });

    let dist0 = dijkstra(0, &g);
    let idx = dist0.iter()
        .position_max()
        .unwrap();

    let dist1 = dijkstra(idx, &g);
    let max = *dist1.iter()
        .max()
        .unwrap();

    println!(
        "{}",
        edges.iter()
            .map(|(_, _, w)| *w)
            .sum::<u64>() * 2 - max
    );
}
