use cps::veclibs::VecLibs;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        k: u64,
        a: [u64; n]
    }

    let ans = a.lower_bound(k);
    if a[n-1] < k {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
