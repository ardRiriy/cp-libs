use std::collections::BTreeSet;

use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut v = vec![0; n];
    let mut set = BTreeSet::from_iter(0..=n);

    for &ai in a[0..m].iter() {
        set.remove(&ai);
        v[ai] += 1;
    }

    let mut ans = *set.iter().next().unwrap();
    for (j, &ai) in a[m..].iter().enumerate() {
        // a[j]をremove
        v[a[j]] -= 1;
        if v[a[j]] == 0 {
            set.insert(a[j]);
        }

        v[ai] += 1;
        set.remove(&ai);
        ans.chmin(*set.iter().next().unwrap());
    }
    println!("{ans}");
}
