use std::collections::BTreeSet;

use ac_library::Dsu;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        a: [Usize1; n],
    }
    let mut uf = Dsu::new(n);
    let mut ans = BTreeSet::new();

    for (i, x) in a.iter().enumerate() {
        if uf.same(i, *x) {
            let mut current = *x;
            loop {
                ans.insert(current);
                if current == i {
                    break;
                } else {
                    current = a[current];
                }
            }
        } else {
            uf.merge(i, *x);
        }
    }

    println!("{}", ans.len());
}

