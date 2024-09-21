use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        s: Chars,
    }

    println!("{}",
        s.iter().filter_map(|c| if *c == '.' { None } else { Some(*c)} ).join("")
    );
}
