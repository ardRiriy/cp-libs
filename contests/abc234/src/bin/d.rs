use ac_library::FenwickTree;
use proconio::input;
fn solve() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }

    let mut bit = FenwickTree::new(n, 0u64);
    for i in 0..k - 1 {
        bit.add(p[i] - 1, 1);
    }

    for i in k - 1..n {
        bit.add(p[i] - 1, 1);

        // bit上で、0からxまでの和がi-k以上であるところがok
        let mut ng = !0;
        let mut ok = n;

        while ok.wrapping_sub(ng) > 1 {
            let mid = ok.wrapping_add(ng) / 2;
            if bit.sum(0..mid) >= (i + 2 - k) as u64 {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        println!("{ok}");
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
