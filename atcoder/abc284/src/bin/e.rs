use proconio::{input, marker::Usize1};
fn solve() {
    input!{
        n: usize,
        m: usize,
    }
    let g = (0..m).fold(vec![vec![];n], |mut acc, _| {
        input!{ a: Usize1, b: Usize1 };
        acc[a].push(b);
        acc[b].push(a);
        acc
    });

    let mut seen = vec![false; n];
    let ans = dfs(&g, &mut seen, 0);
    println!("{}", ans.min(1e6 as u64));

}

fn dfs(g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, pos: usize) -> u64 {
    let mut res = 1;
    seen[pos] = true;

    for &next in &g[pos] {
        if seen[next] {
            continue;
        }

        res += dfs(g, seen, next);
        if res > 10e6 as u64 {
            return res;
        }
    }
    seen[pos] = false;
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

