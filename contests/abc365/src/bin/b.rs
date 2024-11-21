use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }
    let v = a.iter().enumerate().map(|(i, &x)| (x, i)).sorted().collect_vec();

    println!("{}", v[n-2].1 + 1);
}