use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        x: u64,
        mut a: [u64; n]
    }
    a.insert(k, x);
    println!("{}", a.iter().join(" "));

}
