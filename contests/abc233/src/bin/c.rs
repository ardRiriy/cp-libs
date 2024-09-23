use proconio::{input};
fn solve() {
    input!{
        n: usize,
        x: u64,
    }
    let l = (0..n).fold(vec![], |mut acc, _| {
        input! {
            li: usize,
            a: [u64; li]
        }
        acc.push(a);
        acc
    });

    println!("{}", dfs(&l, n, x));

}

fn dfs(l: &Vec<Vec<u64>>, idx: usize, target: u64) -> u64 {
    if idx == 0 {
        return if target == 1 {
            1
        } else {
            0
        };
    }

    let mut res = 0;
    for elem in l[idx-1].iter() {
        if target % elem == 0 {
            res += dfs(l, idx-1, target/elem);
        }
    }

    res
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

