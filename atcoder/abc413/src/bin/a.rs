#[allow(unused_imports)]
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: u64,
        a: [u64; n],
    }

    let s = a.iter().sum::<u64>();
    println!("{}", if s<=m { "Yes" } else { "No" });
}
