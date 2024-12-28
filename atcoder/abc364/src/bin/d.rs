use proconio::{input};
fn solve() {
    input!{
        n: usize,
        q: usize,
        mut a: [i64; n],
    }

    a.sort();

    let bs = |k| -> usize {
        let mut ng = !0;
        let mut ok = n;
        while ok.wrapping_sub(ng) > 1 {
            let mid = ok.wrapping_add(ng) / 2;
            if a[mid] >= k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    };

    let judge = |q: i64, x: i64, k: usize| -> bool {
        let lower = bs(q - x);
        let upper = bs(q + x + 1);
        upper - lower >= k
    };

    for _ in 0..q {
        input! {
            b: i64,
            k: usize
        }

        let mut ok = INF as i64;
        let mut ng = -1;

        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if judge(b, mid, k) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok);
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

