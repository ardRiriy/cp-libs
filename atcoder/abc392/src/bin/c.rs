use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    }
    let ans = (0..n)
        .sorted_unstable_by_key(|i| q[*i])
        .map(|i| 1 + q[p[i]])
        .collect_vec();
    println!("{}", ans.iter().join(" "));
}
