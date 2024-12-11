#[allow(unused_imports)]
use cps::debug::*;
use cps::warshall_floyd::{DefaultWFelm, WarshallFloyd};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        r: usize,
        rv: [Usize1; r],
        e: [(Usize1, Usize1, u64); m],
    }

    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v, w)| { g[u].push((v, w)); g[v].push((u, w)); g });
    let wf = WarshallFloyd::new(&g, DefaultWFelm);

    let ans = rv.iter()
        .copied()
        .permutations(r)
        .map(|v| {
            v.iter().tuple_windows().map(|(from, to)| wf.get(*from, *to)).sum::<u64>()
        })
        .min()
        .unwrap();

    println!("{ans}");
}

