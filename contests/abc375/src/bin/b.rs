use itertools::Itertools;
use libm::hypot;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        mut a: [(f64, f64); n],
    }

    a.insert(0, (0., 0.));
    a.push((0., 0.));

    let ans = a.iter()
        .tuple_windows()
        .map(|(a, b)| hypot(a.0 - b.0, a.1 - b.1))
        .sum::<f64>();
    println!("{ans}");

}
