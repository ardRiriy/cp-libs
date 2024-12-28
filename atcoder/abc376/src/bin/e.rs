use cps::{chlibs::ChLibs, mink_sum::MinK};
use itertools::Itertools;
use proconio::input;

fn solve() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
        b: [u64; n],
    }

    let mut mink = MinK::new(k);

    let sorted = a.iter()
        .enumerate()
        .map(|(idx, &x)| (x, b[idx]))
        .sorted()
        .collect_vec();

    let mut ans = 1u64 << 62;
    for &(ai, bi) in sorted.iter() {
        mink.add(bi);
        if mink.size() >= k {
            ans.chmin(ai * mink.ans());
        }
    }
    println!("{ans}");
}

fn main() {
    input!{
        t: u64,
    }
    for _ in 0..t {
        solve();
    }
}