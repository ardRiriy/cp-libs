use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; m],
        ss: [Chars; n],
    }
    let points = ss
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(|(j, c)| if *c == 'o' { a[j] } else { 0 })
                .sum::<u64>()
                + i as u64
                + 1
        })
        .collect_vec();
    let max = points.iter().copied().max().unwrap();
    let indicates = (0..m)
        .into_iter()
        .sorted_by_key(|i| Reverse(a[*i]))
        .collect_vec();
    let ans = points
        .iter()
        .enumerate()
        .map(|(i, &pi)| {
            return if max == pi {
                0
            } else {
                let mut cnt = 0;
                let mut adder = 0;
                for j in 0..m {
                    if ss[i][indicates[j]] == 'x' {
                        cnt += 1;
                        adder += a[indicates[j]]
                    }
                    if adder + pi > max {
                        break;
                    }
                }
                cnt
            };
        })
        .collect_vec();
    println!("{}", ans.iter().join("\n"));
}
