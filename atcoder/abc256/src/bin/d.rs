use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        ranges: [(usize, usize); n],
    }

    let max = 2 * 1e5 as usize + 3;
    let mut s = vec![0i64; max];

    for (l, r) in ranges {
        s[l] += 1;
        s[r] -= 1;
    }

    let mut current = 0;
    let mut flag = false;
    let mut last = 0;
    let mut ans = vec![];
    for (idx, &x) in s.iter().enumerate() {
        current += x;
        if !flag && current > 0 {
            flag = true;
            last = idx;
        } else if flag && current == 0 {
            ans.push((last, idx));
            flag = false;
        }
    }
    println!("{}", ans.iter().map(|(a, b)| format!("{} {}", a, b)).join("\n"));

}
