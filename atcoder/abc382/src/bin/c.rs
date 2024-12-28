use std::cmp::Reverse;

#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [u64; n],
        b: [u64; m],
    }

    let inf = 1usize << 60;
    let mut ans = vec![inf; m];

    let indicates = (0..m).sorted_by_key(|i| Reverse(b[*i])).collect_vec();

    let mut bi = 0;

    for (i, &ai) in a.iter().enumerate() {
        while bi < m && b[indicates[bi]] >= ai {
            ans[indicates[bi]] = i+1;
            bi += 1;
        }
    }

    println!("{}", ans.iter().map(|&x| if x == inf { -1 } else { x as i64 }).join("\n"));
}

