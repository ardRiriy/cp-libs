#[allow(unused_imports)]
use cps::debug::*;
use cps::potentiality_unionfind::{DefaultPotentialMergeOp, PotentialityUnionfind};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        q: usize,
        queries: [(Usize1, Usize1, i64); q]
    }

    let mut uf = PotentialityUnionfind::new(n, Some(DefaultPotentialMergeOp));
    let mut ans = vec![];
    for (idx, &(u,v,w)) in queries.iter().enumerate() {
        if let Ok(_) = uf.merge(u, v, w) {
            ans.push(idx+1);
        }
    }
    println!("{}", ans.iter().join(" "));
}

