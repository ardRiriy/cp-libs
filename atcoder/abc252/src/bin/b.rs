use std::process::exit;

use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        k: usize,
        a: [u64; n],
        b: [Usize1; k],
    }

    let a = a.iter()
        .enumerate()
        .map(|(idx, &a)| {
            (a, idx)
        })
        .sorted()
        .rev()
        .collect_vec();

    let max = a[0].0;

    for (ai, idx) in a {
        if ai != max {
            break;
        }

        if b.contains(&idx) {
            println!("Yes");
            exit(0)
        }
    }
    println!("No");
}
