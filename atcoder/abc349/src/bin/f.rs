use std::collections::BTreeMap;

use cps::prime::prime_factorization;
use cps::zeta_mobius_transform::{mobius_transform, zeta_transform};
use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: u64,
        a: [u64; n],
    }

    let p = prime_factorization(m);
    let k = p.len();

    let mut c = BTreeMap::new();
    for &v in a.iter() {
        if m % v != 0 {
            continue;
        }

        let mut mask = 0;
        for (i, (k, c)) in p.iter().enumerate() {
            if v % (k.pow(*c as u32)) == 0 {
                mask |= 1<<i;
            }
        }
        *c.entry(mask).or_insert(0) += 1;
    }

    for i in 0..2u64.pow(k as u32) {
        if let None = c.get(&i) {
            c.insert(i,0);
        }
    }
    let f = c.iter()
        .map(|(_,v)| *v)
        .collect_vec();

    let g = zeta_transform(&f, k)
        .iter()
        .map(|v| Mint::new(2).pow(*v))
        .collect_vec();
    
    let ans = mobius_transform(&g, k);

    println!("{}", ans[(1<<p.len()) - 1] - if m==1 { 1 } else { 0 });
}

