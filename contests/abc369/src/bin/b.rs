use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        v: [(i64, char); n]
    }

    let res = |chr: char| -> i64 {
        v.iter()
            .filter_map(|&(x, c)| if c == chr { Some(x) } else { None } )
            .tuple_windows()
            .map(|(x, y)| (x - y).abs())
            .sum::<i64>()
    };

    println!("{}", res('L') + res('R'));
}
