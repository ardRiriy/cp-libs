use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn solve() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n-1],
        c: [u64; n]
    }

    let g = edges.iter().fold(vec![vec![]; n], |mut acc, &(a, b)| {
        acc[a].push(b);
        acc[b].push(a);
        acc
    });

    let mut s = vec![0u64; n];
    let mut f = vec![INF; n];

    // f(0)を求める
    f[0] = dfs(0, 0, &g, &c, &mut vec![false; n], &mut s);
    ans(0, &g, &c, &mut s, &mut f);
    eprintln!("{:?}", f);
    println!("{}", f.iter().min().unwrap());
}

fn ans(pos: usize, g: &Vec<Vec<usize>>, c: &Vec<u64>, s: &mut Vec<u64>, f: &mut Vec<u64>) {
    let sum = g[pos].iter().map(|x| s[*x]).sum::<u64>();
    let tmp = s[pos];
    for &next in &g[pos] {
        if f[next] == INF {
            s[pos] = sum + c[pos] - s[next];
            f[next] = f[pos] + sum + c[pos] - 2 * s[next];
            ans(next, g, c, s, f);
        }
    }
    s[pos] = tmp;
}
fn dfs(
    pos: usize,
    dpt: u64,
    g: &Vec<Vec<usize>>,
    c: &Vec<u64>,
    seen: &mut Vec<bool>,
    s: &mut Vec<u64>,
) -> u64 {
    seen[pos] = true;
    let mut res = dpt * c[pos];
    for &next in &g[pos] {
        if !seen[next] {
            res += dfs(next, dpt + 1, g, c, seen, s);
            s[pos] += s[next];
        }
    }

    s[pos] += c[pos];

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
