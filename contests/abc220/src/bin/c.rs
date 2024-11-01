use cps::cumulative_sum::CumulativeSum;
use proconio::input;
fn main() {
    input!{
        n: usize,
        a: [u64; n],
        x: u64,
    }
    let sum = a.iter().sum::<u64>();
    let csum = CumulativeSum::new(&a);
    let left = x % sum;

    let i = csum.binary_search(left+1).unwrap_or_else(|x| x);
    println!("{}", i + (x / sum) as usize * n);
}

