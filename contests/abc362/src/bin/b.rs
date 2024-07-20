use proconio::{input};
fn solve() {
    input!{
        a: (i64, i64),
        b: (i64, i64),
        c: (i64, i64)
    }

    let d1 = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2);
    let d2 = (c.0 - b.0).pow(2) + (c.1 - b.1).pow(2);
    let d3 = (c.0 - a.0).pow(2) + (c.1 - a.1).pow(2);

    if (d1 + d2 == d3) || (d2 + d3 == d1) || (d3 + d1 == d2) {
        println!("Yes");
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

