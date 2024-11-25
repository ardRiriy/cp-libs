use std::cmp::Reverse;

use ac_library::{Max, Segtree};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        q: usize,
        h: [u64; n],
        mut queries: [(Usize1, Usize1); q],
    }
    let mut ans = vec![0; q];
    let indices = (0..q)
        .sorted_unstable_by_key(|i| Reverse(queries[*i].1))
        .collect_vec();
    queries.sort_unstable_by_key(|&i| Reverse(i.1));
    let mut qi = 0;

    let mut stk = vec![];

    let mut segtree :Segtree<Max<u64>> = Segtree::new(n);
    for (i, &hi) in h.iter().enumerate() {
        segtree.set(i, hi);
    }

    for (i, &hi) in h.iter().enumerate().rev() {
        while qi < q && queries[qi].1 == i {
            let (l, r) = queries[qi];
            let mh = segtree.prod(l+1..=r);

            let mut ng = stk.len();
            let mut ok = !0;
            while ng.wrapping_sub(ok) > 1 {
                let mid = ng.wrapping_add(ok) / 2;
                if stk[mid] < mh {
                    ng = mid;
                } else {
                    ok = mid;
                }
            }
            if ok != !0 {
                ans[indices[qi]] = ok + 1;
            }
            qi += 1;
        }

        while let Some(hj) = stk.pop() {
            if hj > hi {
                stk.push(hj);
                break;
            }
        }

        stk.push(hi);
    }

    println!("{}", ans.iter().join("\n"));
}

