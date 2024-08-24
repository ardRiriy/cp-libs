use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        a: [u64; n]
    }
    let mut memo = BTreeMap::new();
    memo.insert(1, 0);
    let mut xor = 0;
    for &x in a.iter() {
        xor ^= compute_grundy(x, &mut memo);
    }

    if xor == 0 {
        println!("Bruno");
    } else {
        println!("Anna");
    }
}

fn compute_grundy(n: u64, memo: &mut BTreeMap<u64, u64>) -> u64 {
    if let Some(&grundy) = memo.get(&n) {
        return grundy;
    }

    let divisors = get_divisors(n);
    let grundy_values = divisors.iter().map(|&d| compute_grundy(d, memo)).collect_vec();
    let mut grundy = 0;

    while grundy_values.contains(&grundy) {
        grundy += 1;
    }

    memo.insert(n, grundy);
    grundy
}

fn get_divisors(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i != n / i && i != 1{
                res.push(n / i);
            }
        }
        i += 1;
    }
    res
}
