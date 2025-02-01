#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n+1],
        c: [i64; n+m+1],
    }

    let mut b = vec![0; m + 1];
    // set[i]には、未知の値b_iが含まれるとき集合にiを持つ
    let mut set = vec![BTreeSet::new(); n + m + 1];
    let mut que = VecDeque::new();
    for (i, st) in set.iter_mut().enumerate() {
        for k in 0..=m {
            if i < k || i - k > n {
                // そのような組は存在しない
                continue;
            }
            if a[i - k] != 0 {
                st.insert(k);
            }
        }
        if st.len() == 1 {
            que.push_back(i);
        }
    }

    let mut sum = vec![0; n + m + 1];
    while let Some(idx) = que.pop_front() {
        if set[idx].is_empty() {
            continue;
        }
        let i = *set[idx].iter().next().unwrap();
        b[i] = (c[idx] - sum[idx]) / a[idx - i];
        for (j, st) in set.iter_mut().enumerate() {
            if st.remove(&i) {
                sum[j] += b[i] * a[j - i];
                if st.len() == 1 {
                    que.push_back(j);
                }
            }
        }
    }
    println!("{}", b.iter().join(" "));
}
