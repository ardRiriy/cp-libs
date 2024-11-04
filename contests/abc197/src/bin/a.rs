use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        mut s: Chars,
    }
    let c = s.remove(0);
    s.push(c);
    println!("{}", s.iter().join(""));
}
