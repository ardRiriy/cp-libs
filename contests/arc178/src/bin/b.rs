use std::mem::swap;

use ac_library::modint::ModIntBase;
use proconio::input;

fn solve() {
    input! {
        mut a1: usize,
        mut a2: usize,
        a3: usize,
    }

    if a1 > a2 {
        swap(&mut a1, &mut a2)
    }

    if a2 == a3 {
        let mut ans = ac_library::ModInt998244353::new(0);
        let pow_a2 = ac_library::ModInt998244353::new(10).pow(a2 as u64);
        let pow_a1 = ac_library::ModInt998244353::new(10).pow(a1 as u64);

        ans += (pow_a2 - pow_a1) * (pow_a1 - pow_a1 / 10);
        ans += (pow_a1 - 1 + pow_a1 / 10) * (pow_a1 - pow_a1 / 10 - 1) / 2;

        println!("{}", ans);
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
    input! { mut i: usize }
    // let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
