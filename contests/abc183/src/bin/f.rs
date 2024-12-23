use std::collections::BTreeMap;

use ac_library::Dsu;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [u64; n],
    }

    let mut uf = Dsu::new(n);
    let mut v = vec![BTreeMap::new(); n];
    for (idx, &c) in c.iter().enumerate() {
        v[idx].insert(c, 1u64);
    }

    for _ in 0..q {
        input! {
            q: u8,
            a: Usize1,
            b: Usize1,
        }
        if q == 1 {
            if uf.same(a, b) {
                continue;
            }

            let al = uf.leader(a);
            let bl = uf.leader(b);
            if uf.size(a) < uf.size(b) {
                v.swap(al, bl);
            }

            let a_map_ptr = &mut v[al] as *mut BTreeMap<_, _>;
            for (&c, &val) in v[uf.leader(b)].iter() {
                unsafe {
                    *(*a_map_ptr).entry(c).or_insert(0) += val;
                }
            }
            uf.merge(a, b);
            v.swap(al, uf.leader(a));
        } else {
            if let Some(c) = v[uf.leader(a)].get(&u64::try_from(b + 1).unwrap()) {
                println!("{}", c);
            } else {
                println!("{}", 0);
            }
        }
    }
}
