use proconio::input;

fn solve() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
    }
    let mut idx = 0;

    for _ in 0..q {
        input! {
            t: u32,
            x: usize,
            y: usize,
        }

        match t {
            1 => {
                a.swap((idx + (x - 1)) % n, (idx + (y - 1)) % n);
            }
            2 => {
                idx = (idx + n - 1) % n;
            }
            3 => {
                println!("{}", a[(idx + (x - 1)) % n]);
            }
            _ => { unreachable!(); }
        }
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
