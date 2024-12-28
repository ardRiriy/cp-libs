use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: String,
        k: usize
    }

    let mut t = n;
    for _ in 0..k {
        t = operate(t);
    }

    println!("{}", t);
}

fn operate(x: String) -> String {
    // 8 -> 10
    let mut x_base_10 = u64::from_str_radix(&x, 8).unwrap();

    let mut res = vec![];
    loop {
        let num = x_base_10 % 9;
        res.push(num);
        x_base_10 /= 9;
        if x_base_10 == 0 {
            break;
        }
    }

    res.reverse();
    res.iter()
        .map(|&c| if c == 8 { 5 } else { c })
        .join("")
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
