use proconio::input;

fn solve() {
    input! {
        n: usize,
        p: [(i64, i64); n],
    }

    let mut v = vec![vec![vec![]; 2]; 2];

    for &(x, y) in p.iter() {
        let k = (x + y) % 2;
        let mut x = x;

        if k == 1 {
            x -= 1;
        }

        v[k as usize][0].push((x + y) / 2);
        v[k as usize][1].push(-(x - y) / 2);
    }

    let mut ans = 0i64;
    for i in 0..2 {
        for j in 0..2 {
            v[i][j].sort_unstable();

            let mut sum = 0;
            for (idx, &x) in v[i][j].iter().enumerate() {
                ans += x * idx as i64 - sum;
                sum += x;
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
