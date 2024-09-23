use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        papers: [((usize, usize), (usize, usize)); n]
    }

    let k = 1002;
    let mut imos = vec![vec![0i64; k]; k];
    for ((si, sj), (ei, ej)) in papers {
        imos[si][sj] += 1;
        imos[si][ej] -= 1;
        imos[ei][sj] -= 1;
        imos[ei][ej] += 1;
    }

    for i in 0..k {
        for j in 1..k {
            let val = imos[i][j] + imos[i][j-1];
            imos[i][j] = val;
        }
    }

    for j in 0..k {
        for i in 1..k {
            let val = imos[i][j] + imos[i-1][j];
            imos[i][j] = val;
        }
    }

    let mut ans = vec![0u64;n];
    for i in 0..k {
        for j in 0..k {
            if imos[i][j] == 0 { continue; }
            ans[imos[i][j] as usize - 1] += 1;
        }
    }

    println!("{}", ans.iter().join("\n"));

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
