use cps::dijkstra::Dijkstra;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

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

    let d0 = Dijkstra::new(0, &g);
    let idx = (0..n).map(|i| d0.get(i))
        .position_max()
        .unwrap();

    let d1 = Dijkstra::new(idx, &g);
    let max = (0..n)
        .map(|i| d1.get(i))
        .max()
        .unwrap();

    println!(
        "{}",
        edges.iter()
            .map(|(_, _, w)| *w)
            .sum::<u64>() * 2 - max
    );
}
