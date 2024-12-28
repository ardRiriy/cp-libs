use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n]
    }

    let mut vm = vec![BTreeMap::new(); k];

    for (idx, &x) in a.iter().enumerate() {
        *vm[idx % k].entry(x).or_insert(0u64) += 1;
    }
    

    for (idx, &x) in a.iter().sorted().enumerate() {
        let y = match vm[idx % k].get(&x) {
            Some(y) => {
                if *y > 0 { *y - 1 }
                else { println!("No"); return;}
            }
            None => { println!("No"); return;}
        };

        vm[idx%k].insert(x, y);
    }
    println!("Yes");

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
