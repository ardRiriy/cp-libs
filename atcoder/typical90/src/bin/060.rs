use cps::chlibs::ChLibs;
use cps::consts::INF; 
use cps::veclibs::VecLibs;
use itertools::Itertools;
use proconio::input;

fn lis(a: &[u64]) -> Vec<u64> {
    // aのi番目の要素までの最長増加部分の長さを返す
    let n = a.len();
    let mut res = vec![INF; n];
    let mut dp = vec![INF; n];
    let mut len = 0;
    for (i, ai) in a.iter().enumerate() {
        let idx = dp.lower_bound(*ai);
        dp[idx] = *ai;
        len.chmax(idx+1);
        res[i] = len as u64;
    }

    return res;
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let rev_a = a.iter().copied().rev().collect_vec();

    let va = lis(&a);
    let vr = lis(&rev_a).iter().copied().rev().collect_vec();
    println!("{}", (0..n).map(|i| va[i]+vr[i]-1).max().unwrap());
}