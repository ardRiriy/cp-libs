use std::collections::BTreeMap;

use itertools::iproduct;
use proconio::{input};
fn solve() {
    input!{
        n: usize,
        x: usize,
        y: u64,
        mut v: [(usize, u64); n]
    }

    v.sort_by_key(|(_, yi)| *yi);

    // dp[i][j] := i品食べて甘さの合計がjであるとき、しょっぱさの最小値
    let mut dp = vec![vec![INF; x + 2]; n+1];
    dp[0][0] = 0;

    for idx in 0..n {
        let (xi, yi) = v[idx];

        for (i, j) in iproduct!((0..=idx).rev(), (0..x+1).rev()) {
            if dp[i][j] > y { continue; }

            let val = dp[i][j] + yi;
            dp[i+1][(j + xi).min(x+1)].chmin(val);
        }
    }

    let mut ans = 0;
    for i in 0..n+1 {
        for j in 0..x+2 {
            if dp[i][j] != INF {
                ans = i;
                break;
            }
        }
    }
    println!("{ans}");

}

/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/

static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        return if *self > elm {
            *self = elm;
            true
        } else {
            false
        };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return if *self < elm {
            *self = elm;
            true
        } else {
            false
        };
    }
}

fn main() {
    // input! { mut i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}

