use itertools::Itertools;
use proconio::{input, marker::Chars};
fn solve() {
    input!{
        n: usize,
        t: i64,
        s: Chars,
        x: [i64; n]
    }
    let mut x = x.iter().enumerate().map(|(idx, x)| (*x, idx)).collect_vec();
    x.sort_by_key(|(x, _)| *x);

    let migi = x
        .iter()
        .filter_map(|&(x, idx)| if s[idx] == '1' { Some(x) } else { None })
        .collect_vec();

    let before = x
        .iter()
        .filter_map(|&(x, idx)| if s[idx] == '0' { Some(x) } else { None })
        .collect_vec();

    let after = before.iter().map(|&x| x - t).collect_vec();

    if before.is_empty() || migi.is_empty() {
        println!("0");
        return;
    }
    let mut l = 0;
    let mut r = 0;
    let mut ans = 0;

    // println!("{:?}", migi);
    // println!("{:?}", before);
    // println!("{:?}", after);

    for &xl in migi.iter() {
        let xr = xl + t;
        while r < after.len() && xr >= after[r] {
            r += 1;
        }

        while l < before.len() && xl >= before[l] {
            l += 1;
        }

        // println!("{}", before[r]);
        // println!("{} {}", xl, xr);
        // println!("{} {}", l, r);
        ans += r - l;
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
    // input! { mut i: usize }
    let mut i = 1;
    while i != 0 {
        solve();
        i -= 1;
    }
}

