use itertools::Itertools;
use proconio::{input, marker::{Chars, Usize1}};

fn main() {
    input!{
        _n: usize,
        q: usize,
        s: Chars,
    }
    let v = s.iter()
        .tuple_windows()
        .map(|(&c, &d)| if c == d { 1 } else { 0 })
        .fold(vec![0u64], |mut v, i| { v.push(v[v.len()-1] + i); v });

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
        }
        println!("{}", v[r] - v[l]);
    }
}

