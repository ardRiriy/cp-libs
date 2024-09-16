use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
    }

    let a = (0..n)
        .map(|idx| {
            input! {
                v: [Usize1; idx+1],
            }
            v
        })
        .collect_vec();

    let mut c = 0;
    for i in 0..n {
        c = a[c.max(i)][c.min(i)];
    }
    println!("{}", c+1);
}
