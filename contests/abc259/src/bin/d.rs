use ac_library::Dsu;
use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        start: (i64, i64),
        goal: (i64, i64),
        circle: [(i64, i64, i64); n]
    }

    let check = |p1: (i64, i64, i64), p2: (i64, i64, i64)| {
        if p1.0 == p2.0 && p1.1 == p2.1 {
            return p1.2 == p2.2;
        }
        let l = (p1.2 + p2.2) * (p1.2 + p2.2);
        let r = (p1.0 - p2.0) * (p1.0 - p2.0) + (p1.1 - p2.1) * (p1.1 - p2.1);
        l >= r
    };

    let circle = circle.iter().enumerate().map(|(idx, x)| (idx, *x)).collect_vec();

    // n -> start, n+1 -> goal
    let mut uf = circle.iter()
        .tuple_combinations()
        .fold(Dsu::new(n+2),
        |mut uf, (&p1, &p2)| {
            if check(p1.1, p2.1) {
                uf.merge(p1.0, p2.0); 
            }
            uf
        });

    for &(idx, p) in circle.iter() {
        if (start.0 - p.0) * (start.0 - p.0) + (start.1 - p.1) * (start.1 - p.1)== p.2 * p.2{
            uf.merge(idx, n);
        }
        if (goal.0 - p.0) * (goal.0 - p.0) + (goal.1 - p.1) * (goal.1 - p.1) == p.2 * p.2{
            uf.merge(idx, n+1);
        }
    }
    
    println!("{}", if uf.same(n, n+1) {"Yes"} else {"No"})



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
