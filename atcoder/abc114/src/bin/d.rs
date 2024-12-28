use std::collections::BTreeMap;

#[allow(unused_imports)]
use cps::debug::*;
use cps::prime::prime_factorization;
use proconio::input;

fn main() {
    input!{
        n: usize,
    }

    let mut counts = vec![0; n+1];
    
    for i in 1..=n {
        let res = prime_factorization(i as u64);
        for (key, val) in res {
            counts[key as usize] += val;
        }
    }
    

    let calc = |key: u64| -> usize {
        counts.iter().filter(|&x| *x >= key).count()
    };

    let cnt2 = calc(2);
    let cnt4 = calc(4);
    let cnt14 = calc(14);
    let cnt24 = calc(24);
    let cnt74 = calc(74);

    let mut ans = cnt74; // a^74
    if cnt2 > 1 {
        ans += cnt24 * (cnt2 - 1); // a^24 * b^2
    }
    if cnt4 > 1 {
        ans += cnt14 * (cnt4 - 1);
    }
    if cnt4 > 1 && cnt2 > 2 {
        ans += cnt4 * (cnt4-1) * (cnt2-2) / 2;
    }
    println!("{ans}");
}

