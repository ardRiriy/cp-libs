use cps::cumulative_sum::CumulativeSum;
use proconio::input;
fn main() {
    input!{
        n: usize,
        k: usize,
        a: [u64; n]
    }

    let csum = CumulativeSum::new(&a);
    let ans = (0..n-k+1).map(|i| {
        csum.get(i..i+k)
    }).sum::<u64>();
    println!("{}", ans);
}
