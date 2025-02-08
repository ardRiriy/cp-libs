use itertools::Itertools;
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let set = BTreeSet::from_iter(a.iter().copied());
    let ans = (1..=n).filter(|i| !set.contains(i)).collect_vec();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
