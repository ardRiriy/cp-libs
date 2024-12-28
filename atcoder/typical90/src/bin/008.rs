use ac_library::ModInt1000000007;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn solve() {
    input! {
        n: usize,
        s: Chars
    }

    let atcoder = "atcoder".chars().collect_vec();
    let mut dp = vec![vec![ModInt1000000007::new(0); atcoder.len()]; n+1];

    for (idx, c) in s.iter().enumerate() {
        if let Some((j, c)) = atcoder.iter().enumerate().find(|(_, &d)| *c == d) {
            if *c == 'a' {
                dp[idx][0] += 1;
            } else {
                let tmp = dp[idx][j-1];
                dp[idx][j] += tmp;
            }
        }
        dp[idx+1] = dp[idx].clone();
    }

    // println!("{:?}", dp);
    println!("{}", dp[n][atcoder.len()-1]);

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
