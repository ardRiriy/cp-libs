use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }

    println!("{}", if a.iter().tuple_windows().all(|(a, b)| a < b) {
        "Yes"
    } else {
        "No"
    });
}
