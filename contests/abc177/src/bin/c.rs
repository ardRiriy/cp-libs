use proconio::input;

fn solve() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let cs = CumulativeSum::new(&a);
    let ans = (1..=n).into_iter().fold(
        ModInt1000000007::from(0), |ans, x| {
            ans + ModInt1000000007::from(cs.get(x..n)) * ModInt1000000007::from(a[x-1])
        }
    );
    println!("{ans}");
}

use std::ops::Range;
use ac_library::{ModInt1000000007};
use num_traits::Num;

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
        return
            if *self > elm {
                *self = elm;
                true
            } else { false };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return
            if *self < elm {
                *self = elm;
                true
            } else { false };
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
