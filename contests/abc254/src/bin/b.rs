use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize
    }

    let mut a = vec![vec![1]; n];
    for i in 1..n {
        for j in 1..i {
            let k = a[i-1][j-1] + a[i-1][j];
            a[i].push(k);
        }
        a[i].push(1);
    }
    println!("{}", a.iter().map(|v| v.iter().join(" ")).join("\n"));

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
