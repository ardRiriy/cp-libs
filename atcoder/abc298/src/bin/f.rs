use std::collections::BTreeMap;

use ac_library::{Max, Segtree};
use cps::chlibs::ChLibs;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        p: [(usize, usize, u64); n],
    }

    let mut row_zaatu_conter = 0;
    let mut row_zaatu = BTreeMap::new();
    let mut row_points = BTreeMap::new();
    let mut row_sum = vec![0; n];

    for &(r, c, w) in p.iter().sorted_by_key(|&(r, _, _)| r) {
        row_points.entry(r).or_insert_with(|| Vec::new()).push((c, w));
        if let Some(i) = row_zaatu.get(&r) {
            row_sum[*i] += w;
        } else {
            row_sum[row_zaatu_conter] += w;
            row_zaatu.insert(r, row_zaatu_conter);
            row_zaatu_conter += 1;
        }
    }

    let mut col_zaatu_counter = 0;
    let mut col_zaatu = BTreeMap::new();
    let mut col_sum = vec![0; n];
    for &(_, c, w) in p.iter().sorted_by_key(|&(_, c, _)| c) {
        if let Some(i) = col_zaatu.get(&c) {
            col_sum[*i] += w; 
        } else {
            col_sum[col_zaatu_counter] += w;
            col_zaatu.insert(c, col_zaatu_counter);
            col_zaatu_counter += 1;
        }
    }

    let mut seg :Segtree<Max<u64>> = Segtree::new(n);
    for (i, ci) in col_sum.iter().enumerate() {
        seg.set(i, *ci);
    }

    let mut ans = 0;
    for (r, r_zaatued_i) in row_zaatu.iter() {
        let v = row_points.get(r).unwrap();

        for (c, wi) in v {
            let i = col_zaatu.get(c).unwrap();

            seg.set(*i, col_sum[*i] - *wi);
        }

        ans.chmax(row_sum[*r_zaatued_i] + seg.all_prod());

        for (c, _) in v {
            let i = col_zaatu.get(c).unwrap();
            seg.set(*i, col_sum[*i]);
        }
    }

    println!("{}", ans);
}

