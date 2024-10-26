use cps::cumulative_sum::CumulativeSum;
use itertools::Itertools;
use proconio::input;
fn main() {
    input!{
        n: usize,
        q: usize,
        mut r: [u64;n],
        x: [u64; q]
    }

    r.sort_unstable();
    let cs = CumulativeSum::new(&r);
    println!("{}", x.iter()
            .map(
                |qi| match cs.binary_search(*qi) {
                    Ok(i) => i,
                    Err(i) => i-1,
                }
            )
            .join("\n")
    );
}
