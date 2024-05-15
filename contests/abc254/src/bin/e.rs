use std::collections::{BTreeSet, VecDeque};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn solve() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
        q: usize
    }

    let g = edges.iter().fold(vec![vec![]; n], 
            |mut g, &(u, v)| {
                g[u].push(v);
                g[v].push(u);
                g
            } 
        );

    let mut dist = vec![vec![]; n];
    let mut set = BTreeSet::new();
    let mut que = VecDeque::new();

    for _ in 0..q {
        input! {
            x: Usize1,
            k: usize
        }
 
        if dist[x].is_empty() {
            let mut res = vec![0u64; 4];
            que.push_back((0, x));
            while let Some((d, x)) = que.pop_front() {
                if set.contains(&x) { continue;}

                set.insert(x);
                res[d] += x as u64 + 1;

                if d == 3 { continue; }

                for ni in g[x].iter() {
                    if !set.contains(ni) {
                        que.push_back((d + 1, *ni));
                    }
                }
            }
            dist[x] = res;
            set.clear();
        }

        let ans = dist[x]
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| if idx <= k { Some(*x) } else { None })
            .sum::<u64>();
        println!("{}", ans);

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
