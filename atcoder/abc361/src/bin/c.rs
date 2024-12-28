use cps::{chlibs::ChLibs, consts::INF};
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u64; n]
    }
    a.sort();

    let m = n - k;

    let mut l = 0;
    let mut r = m-1;
    let mut ans = INF;
    while r < n {
        ans.chmin(a[r] - a[l]);
        r += 1;
        l += 1;
    }

    println!("{ans}");
}
