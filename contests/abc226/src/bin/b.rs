use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        n: usize,
    }
    let arr = (0..n).into_iter()
        .map(|_|{
            input!{
                l: usize,
                v: [u64; l],
            }
            v
        })
        .collect_vec();

    let set :BTreeSet<Vec<u64>> = arr.iter()
        .cloned()
        .collect();

    println!("{}", set.len());
}
