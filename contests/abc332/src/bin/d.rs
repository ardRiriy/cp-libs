use itertools::{iproduct, Itertools};
use proconio::input;

fn solve() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h],
        b: [[u64; w]; h],
    }

    let mut ans = INF;
    'perm: for (iv, jv) in iproduct!((0..h).permutations(h), (0..w).permutations(w)) {
        for ((bi, &i), (bj, &j)) in iproduct!(iv.iter().enumerate(), jv.iter().enumerate()) {
            if a[i][j] != b[bi][bj] {
                continue 'perm;
            }
        }

        let mut tmp = 0;
        for i in 0..h {
            for j in i + 1..h {
                // i < j
                if iv[i] > iv[j] {
                    tmp += 1;
                }
            }
        }
        for i in 0..w {
            for j in i + 1..w {
                // i < j
                if jv[i] > jv[j] {
                    tmp += 1;
                }
            }
        }
        ans.chmin(tmp);
    }
    println!("{}", if ans == INF { -1 } else { ans as isize });
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
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
