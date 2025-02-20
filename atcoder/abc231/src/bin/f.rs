use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}, hash::Hash};

use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
        b: [u64; n],
    }
    let indicate = (0..n).sorted_by_key(|i| (Reverse(a[*i]), b[*i], *i)).collect_vec();
    let mut bit = FenwickTree::new(n, 0i64);
    let mut map = HashMap::new();
    let sorted_b = b.iter().copied().sorted().collect_vec();
    for i in 0..n {
        map.entry(sorted_b[i]).or_insert(BinaryHeap::new()).push(Reverse(i));
        bit.add(i, 1);
    }

    let mut ans = 0;
    for i in indicate {
        let Reverse(idx) = map.get_mut(&b[i]).unwrap().pop().unwrap();
        ans += bit.sum(idx..);
        bit.add(idx, -1);
    }

    let mut a_map :HashMap<u64, HashMap<u64, i64>>= HashMap::new();
    for i in 0..n {
        if let Some(mp) = a_map.get(&a[i]) {
            if let Some(val) = mp.get(&b[i]) {
                ans += *val;
            }
        }

        *(a_map.entry(a[i]).or_insert(HashMap::new())).entry(b[i]).or_insert(0) += 1;
    }



    println!("{}", ans);
}
