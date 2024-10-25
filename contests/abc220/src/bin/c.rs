use cps::cumulative_sum::CumulativeSum;
use cps::veclibs::VecLibs;
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

    let i = csum.sum.lower_bound(left+1);
    println!("{}", i + (x / sum) as usize * n);
}

