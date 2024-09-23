use proconio::input;

fn solve() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let sum = a.iter().sum::<u64>();
    let target = if sum % 10 == 0 {
        sum / 10
    } else {
        println!("No");
        return;
    };
    
    let mut l = 0;
    let mut r = 0;
    let mut size = 0;

    loop {
        if size < target {
            size += a[r];
            r += 1;
            if r >= n {
                r %= n;
            }
        } else if size == target {
            println!("Yes");
            return;
        } else {
            size -= a[l];
            l += 1;
            if l >= n {
                break;
            }
        }
        // println!("{} {} {}", l, r, size);
    }
    println!("No");
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
