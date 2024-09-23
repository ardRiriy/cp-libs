use std::ops::Range;
use itertools::Itertools;
use num_traits::Num;
use proconio::input;
use proconio::marker::Usize1;

fn solve() {
    input! {
        n: usize,
        score: [(i32, u64); n],
        q: usize
    }

    let v1 = score.iter().map(|&(c, s)| if c == 1 { s } else { 0 }).collect_vec();
    let v2 = score.iter().map(|&(c, s)| if c == 1 { 0 } else { s }).collect_vec();

    let cs1 = CumulativeSum::new(&v1);
    let cs2 = CumulativeSum::new(&v2);

    let ans = (0..q).map(|_| {
        input! {
            a: Usize1,
            b: usize,
        }
        (cs1.get(a..b), cs2.get(a..b))
    }).collect_vec();

    println!("{}", ans.iter().map(|&(a, b)| format!("{a} {b}")).join("\n"))
}

struct CumulativeSum<T> {
    sum: Vec<T>
}

impl<T> CumulativeSum<T> where T: Num + Clone {
    fn new(init_vec: &Vec<T>) -> CumulativeSum<T> {
        let size = init_vec.len();

        let sum = init_vec
            .iter()
            .enumerate()
            .fold(vec![T::zero()], |mut sum, (idx, x)| {
                let next :T = sum[idx].clone() + x.clone();
                sum.push(next);
                sum
            });
        CumulativeSum { sum }
    }

    // 区間[l, r)の総和を答える
    fn get(&self, range: Range<usize>) -> T {
        self.sum[range.end].clone() - self.sum[range.start].clone()
    }
}

mod tests {
    use crate::CumulativeSum;
    fn cumulative_sum_get() {
        let v = vec![1, 3, 8];
        let cs = CumulativeSum::new(&v);
        assert_eq!(1 + 3 + 8, cs.get(0..3));
        assert_eq!(1 + 3, cs.get(0..2));
        assert_eq!(3 + 8, cs.get(1..3));
        assert_eq!(3, cs.get(1..2));
    }
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
            } else { false }
    }

    fn chmax(&mut self, elm: T) -> bool {
        if *self < elm {
                *self = elm;
                true
            } else { false }
    }
}


fn main() {
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
