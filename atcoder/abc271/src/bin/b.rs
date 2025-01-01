use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let a = (0..n).fold(vec![], |mut acc, _| {
        input! {
            l: usize,
            a: [u64; l]
        }
        acc.push(a);
        acc
    });

    let ans = (0..q).map(|_| {
        input! {
            s: Usize1,
            t: Usize1,
        }
        a[s][t]
    }).join("\n");

    println!("{}", ans);
}
