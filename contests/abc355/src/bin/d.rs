use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        v: [(u64, u64); n]
    }

    let event = v
        .iter()
        .flat_map(|&(l, r)| vec![(l, 0u64), (r, 1u64)])
        .sorted()
        .collect_vec();

    let (_, ans) = event
        .iter()
        .fold((0u64, 0u64), |(mut acc, mut ans), &(a, t)| {
            if t == 0 {
                acc += 1;
            } else {
                acc -= 1;
                ans += acc;
            }
            (acc, ans)
        });

    println!("{ans}");
}

fn solve2() {
    input! {
        n: usize,
        mut rv: [(u64, u64); n]
    }

    let mut l_sorted = rv.clone();
    l_sorted.sort_by_key(|(a, _)| *a);

    rv.sort_by_key(|(a, v)| (*v, *a));
    let mut ans = 0;

    for (idx, &(l, r)) in rv.iter().enumerate() {
        let left = {
            // l min mitukeru
            let mut ng = -1;
            let mut ok = n as isize;

            while (ng - ok).abs() > 1 {
                let mid = (ng + ok) / 2;
                if l_sorted[mid as usize].0 >= l {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        let right = {
            let mut ng = -1;
            let mut ok = n as isize;
            while (ng - ok).abs() > 1 {
                let mid = (ng + ok) / 2;
                if l_sorted[mid as usize].0 > r {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        ans += right - left - 1;
    }

    println!("{}", ans);
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
    // input! { i: usize }
    let mut i = 1;
    while i != 0 {
        // solve();
        solve2();
        i -= 1;
    }
}
