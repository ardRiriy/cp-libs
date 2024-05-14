use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn solve() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m]
    }
    let mut dsu = Dsu::new(n);
    let g = edges.iter().fold(
        vec![vec![]; n], |mut g, &(u, v)| {
            g[v].push(u);
            g[u].push(v);
            dsu.merge(u, v);
            g
        });

    if (1..n).into_iter().all(|u| dsu.same(u, 0))
        && n == m + 1
        && g.iter().filter(|v| v.len() == 1).count() == 2
        && g.iter().filter(|v| v.len() == 2).count() == n-2 {
            println!("Yes");
    } else {
        println!("No");
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
