use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let m = 1e5 as usize;
    let op = (0..m)
        .map(|num| {
            (num.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .sum::<usize>() + num) % m
        })
        .collect_vec();

    let mut doubling = vec![vec![0; m]; 60];
    doubling[0] = op;

    for i in 1..60 {
        for j in 0..m {
            doubling[i][j] = doubling[i-1][doubling[i-1][j]];
        }
    }

    let mut ans = n;
    for i in 0..61 {
        if k >> i & 1 == 1 {
            ans = doubling[i][ans];
        }
    }

    println!("{ans}");
}
