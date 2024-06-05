use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn solve() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let keys  = (0..m).into_iter().map(|_| {
        input! {
            c: usize,
            a: [Usize1; c],
            r: char
        }
        (a, r == 'o')
    }).collect_vec();

    let mut ans = 0;

    'i: for i in 0..1<<n {
        for (v, r) in &keys {
            let num = v.iter().filter(|&&x| i >> x & 1 == 1).count();
            if !((num >= k && *r) || ( num < k && !*r)) {
                continue 'i;
            }
        }
        ans += 1;
    }

    println!("{ans}");

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
