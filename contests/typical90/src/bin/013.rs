use cps::dijkstra::Dijkstra;
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
    let d0 = Dijkstra::new(0, &g);
    let d1 = Dijkstra::new(n-1, &g);

    println!("{}", (0..n).map(|idx| d0.get(idx) + d1.get(idx)).join("\n"));
}
