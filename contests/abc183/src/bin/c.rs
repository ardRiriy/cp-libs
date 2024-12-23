#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        t: [[u64; n]; n],
    }

    let ans = (1..n)
        .permutations(n - 1)
        .map(|v| {
            t[0][v[0]]
                + v.iter()
                    .tuple_windows()
                    .map(|(i, j)| t[*i][*j])
                    .sum::<u64>()
                + t[*v.last().unwrap()][0]
        })
        .filter(|s| *s == k)
        .count();
    println!("{}", ans);
}
