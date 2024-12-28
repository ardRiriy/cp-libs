use proconio::input;

fn solve() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
        c: [u64; n]
    }

    let conv = |v: &Vec<u64>| -> Vec<u64> {
        v.iter()
            .fold(vec![0; 46], |mut acc, x| {
                acc[(*x % 46) as usize] += 1;
                acc
            })
    };

    let a = conv(&a);
    let b = conv(&b);
    let c = conv(&c);
    let mut ans = 0;
    for (i, ai) in a.iter().enumerate() {
        for (j, bi) in b.iter().enumerate() {
            for (k, ci) in c.iter().enumerate() {
                if (i + j + k) % 46 == 0 {
                    ans += *ai * *bi * *ci;
                }
            }
        }
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
