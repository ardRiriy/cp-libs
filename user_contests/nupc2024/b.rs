use proconio::*;
use itertools::Itertools;
fn main() {
    input!{
        n: usize,
        m: usize,
        c: [u128; n],
        a: [u128; n],
    }
    let sum = c.iter()
        .enumerate()
        .map(|(i, &ci)| ci * a[i])
        .sum::<u128>();
    let mut left = sum - m as u128;
    let mut res = vec![0; n];
    for i in (0..n).rev() {
        if left % c[i] == 0 {
            res[i] += left / c[i];
            left = left % c[i];
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        if res[i] >= a[i] {
            ans[i] = res[i] - a[i];
        }
    }
    println!("{}", ans.iter().join(" "));
}