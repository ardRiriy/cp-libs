use cps::cumulative_sum::CumulativeSum;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        x: [i64; n],
        p: [u64; n],
        q: usize,
    }

    let csum = CumulativeSum::new(&p);

    for _ in 0..q {
        input! {
            l: i64,
            r: i64,
        }

        let li = x.binary_search(&l).unwrap_or_else(|x| x);
        let ri = x.binary_search(&(r+1)).unwrap_or_else(|x|x);
        println!("{}", csum.get(li..ri));
    }
}
