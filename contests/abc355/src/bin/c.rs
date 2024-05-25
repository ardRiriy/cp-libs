use proconio::input;

fn solve() {
    input! {
        n: usize,
        t: usize,
        a: [usize; t]
    }

    let mut tate = vec![0; n];
    let mut yoko = vec![0; n];
    let mut naname = vec![0; 2];
    for (idx, &x) in a.iter().enumerate() {
        let i = (x - 1) / n;
        let j = (x - 1) % n;
        // println!("{x} {i} {j}");
        tate[i] += 1;
        yoko[j] += 1;

        if i == j {
            naname[0] += 1;
        }
        if n == i + j + 1 {
            naname[1] += 1;
        }
        // println!("{}", naname[1]);
        if tate[i] == n || yoko[j] == n || naname[0] == n || naname[1] == n {
            println!("{}", idx + 1);
            return;
        }
    }
    println!("-1");
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
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
