use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        h: [u64; n]
    }

    let mut stk = vec![];
    let mut ans = vec![];
    for &x in h.iter().rev() {
        ans.push(stk.len());
        while let Some(y) = stk.pop() {
            if y > x {
                stk.push(y);
                break;
            }
        }
        stk.push(x);
    }

    println!("{}", ans.iter().rev().join(" "));
}
