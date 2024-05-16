use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        mut a: [u64; n]
    }

    a.sort();
    let mut que = VecDeque::from(a);
    let mut next = 1;

    'lp: while let Some(x) = que.pop_front() {
        if x < next {
            que.push_back(INF);
            while let Some(k) = que.pop_front() {
                if k < next {
                    que.push_back(INF);
                } else {
                    que.push_front(k);
                    break;
                }
            }
        } else if x > next {
            que.push_front(x); 
            for _ in 0..2 {
                if let None = que.pop_back() {
                    break 'lp;
                }
            }
            que.push_front(next);
        } else {
            next += 1;
        }
    }

    println!("{}", next - 1);


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

