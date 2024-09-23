
use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        a: [u64; n]
    }

    let (_, r) = a.iter().enumerate().fold(
        (vec![INF; n], vec![INF as usize; n]),
        |(mut dp, mut id), (idx, &x)| {
            let mut ng = !0;
            let mut ok = n;
            while (ok.wrapping_sub(ng)) > 1 {
                let mid = (ok.wrapping_add(ng)) / 2;
                if dp[mid] >= x {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            id[idx] = ok;
            dp[ok] = x;
            (dp, id)
        },
    );

    let mut b = a.iter().map(|x| INF - *x).collect_vec();
    b.reverse();
    let (_, mut l) = b.iter().enumerate().fold(
        (vec![INF; n], vec![INF as usize; n]),
        |(mut dp, mut id), (idx, &x)| {
            let mut ng = !0;
            let mut ok = n;
            while (ok.wrapping_sub(ng)) > 1 {
                let mid = (ok.wrapping_add(ng)) / 2;
                if dp[mid] >= x {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            id[idx] = ok;
            dp[ok] = x;
            (dp, id)
        },
    );

    l.reverse();
    let lis = r.iter().max().unwrap();
    let mut ans = vec![];
    for i in 0..n {
        if l[i] + r[i] + 1 == *lis + 1 {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
    println!("{:?} {:?}", l, r);
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
    input! { mut i: usize }
    // let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}
