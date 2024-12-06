#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
static N :usize = 1e5 as usize;

fn main() {
    input!{
        n: usize,
        a: [Usize1; n],
    }
    let v = a.iter()
        .fold(vec![0; N], |mut v, &ai|{ v[ai] += 1; v});
    let unique = a.iter()
        .sorted()
        .dedup()
        .count();
    let k = v.iter()
        .filter(|&vi| *vi != 0 && *vi % 2 == 0)
        .count();
    println!("{}", unique - k % 2);
}

