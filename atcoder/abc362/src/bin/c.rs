use itertools::Itertools;
use proconio::{input};
fn solve() {
    input!{
        n: usize,
        ranges: [(i64, i64); n]
    }

    let sum_l = ranges.iter().map(|x| x.0).sum::<i64>();
    let sum_r = ranges.iter().map(|x| x.1).sum::<i64>();

    if sum_l <= 0 && sum_r >= 0 {
        println!("Yes");
        let v = find_x_values(&ranges).unwrap();
        println!("{}", v.iter().join(" "));

    } else {
        println!("No");
    }
}

fn find_x_values(lr_pairs: &[(i64, i64)]) -> Option<Vec<i64>> {
    let n = lr_pairs.len();
    
    let mut l_sum = 0;
    let mut r_sum = 0;
    for &(l, r) in lr_pairs {
        l_sum += l;
        r_sum += r;
    }

    if l_sum > 0 || r_sum < 0 {
        return None;
    }

    let mut x_values: Vec<i64> = lr_pairs.iter().map(|&(l, _)| l).collect();
    let mut current_sum: i64 = l_sum;

    for i in 0..n {
        let (l, r) = lr_pairs[i];
        if current_sum == 0 {
            break;
        }

        let diff = r - l;
        let needed = -current_sum;

        if needed <= diff {
            x_values[i] += needed;
            current_sum += needed;
        } else {
            x_values[i] += diff;
            current_sum += diff;
        }
    }

    Some(x_values)
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
        if *self > elm {
            *self = elm;
            true
        } else {
            false
        }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
            *self = elm;
            true
        } else {
            false
        }
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

