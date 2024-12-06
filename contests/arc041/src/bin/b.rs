use cps::consts::{DI, DJ};
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut v = s.iter()
        .map(|cv| cv.iter().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let mut ans = vec![vec![0; w]; h];

    for i in 1..h-1 {
        for j in 1..w-1 {
            ans[i][j] = v[i-1][j];
            for r in 0..4 {
                let ni = i.wrapping_add(DI[r]);
                let nj = j.wrapping_add(DJ[r]);
                v[ni][nj] -= ans[i][j];
            }
        }
    }
    println!("{}", ans.iter().map(|v|v.iter().join("")).join("\n"));
}

