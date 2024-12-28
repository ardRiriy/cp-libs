use itertools::Itertools;
use proconio::input;

fn solve(n: usize, v: &[i64]) -> i64 {
    let half = v[n / 2];
    let res = v.iter()
        .map(|x| (*x - half).abs())
        .sum::<i64>();

    res
}

fn main() {
    input! {
        n: usize,
        pos: [(i64, i64); n]
    }
    let pos_x = pos.iter()
        .map(|(x, _)| *x)
        .sorted()
        .collect_vec();

    let pos_y = pos.iter()
        .map(|(_, y)| *y)
        .sorted()
        .collect_vec();

    let ans = solve(n, &pos_x) + solve(n, &pos_y);

    println!("{ans}");
}
