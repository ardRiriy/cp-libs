use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        k: usize,
        a: [u64; n]
    }

    let prefix = &a[0..n-k];
    let suffix = &a[n-k..];
    let v = [suffix, prefix].concat();
    println!("{}", v.iter().join(" "));
}
