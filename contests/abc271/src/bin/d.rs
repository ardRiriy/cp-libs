use itertools::Itertools;
use proconio::{input};
fn solve() {
    input!{
        n: usize,
        s: usize,
        cards: [(usize, usize); n]
    }

    let mut dp = vec![(false, vec![]); s+1];
    let mut next = vec![(false, vec![]); s+1];

    dp[0].0 = true;

    for &(a, b) in &cards {
        next.fill((false, vec![]));
        for i in (0..s).rev() {
            if !dp[i].0 { continue; }

            if a + i <= s {
                next[a+i] = dp[i].clone();
                next[a+i].1.push('H');
            }

            if b + i <= s {
                next[b+i] = dp[i].clone();
                next[b+i].1.push('T');
            }
        }
        dp = next.clone();
        // println!("{:?}", dp);
    }

    if dp[s].0 {
        println!("Yes");
        println!("{}", dp[s].1.iter().join(""));
    } else {
        println!("No");
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

