use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        l: Usize1,
        r: Usize1
    }

    let mut ans = (1..=n).collect_vec();
    ans[l..=r].reverse();
    println!("{}", ans.iter().join(" "))
}
