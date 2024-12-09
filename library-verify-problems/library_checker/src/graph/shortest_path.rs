// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path
use cps::dijkstra::Dijkstra;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        e: [(usize, usize, u64); m],
    }
    
    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v, w)| { g[u].push((v, w)); g });
    
    let dijkstra = Dijkstra::new(s, &g);
    match dijkstra.path(t) {
        Some(v) => {
            println!("{} {}", dijkstra.get(t), v.len()-1);
            println!("{}", v.iter().tuple_windows().map(|(u, v)| format!("{u} {v}")).join("\n"));
        }
        None => { println!("-1") }
    }
}
