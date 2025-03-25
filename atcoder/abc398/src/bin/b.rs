#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    let n = 7;
    input!{
        v: [Usize1; n],
    }


    for v in v.iter().copied().combinations(5) {
        let mut cnt = vec![0; 14];
        for vi in v {
            cnt[vi] += 1;
        }
        cnt.sort();
        cnt.reverse();
        if cnt[0] == 3 && cnt[1] == 2 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

