use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        m: usize,
    }

    let mut k = 1;
    let mut t = 0;
    while k < m {
        k *= 3;
        t += 1;
    }

    let mut ans = vec![];
    let mut current = m;
    while current > 0 {
        if k <= current {
            ans.push(t);
            current -= k;
        } else {
            t -= 1;
            k /= 3;
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
