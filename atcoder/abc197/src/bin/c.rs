use cps::chlibs::ChLibs;
use cps::consts::INF;
use proconio::{input};
fn main() {
    input!{
        n: usize,
        a: [u32; n]
    }
    println!("{}", dfs(&a, 0, 0));
}

fn dfs(a: &[u32], idx: usize, or_sum: u32) -> u32 {
    if idx == a.len() {
        return or_sum;
    }
    let mut res = INF as u32;
    // or_sumに新たに加える
    res.chmin(dfs(a, idx+1, or_sum | a[idx]));

    // 新しくXORに加える
    res.chmin(or_sum ^ (dfs(a, idx+1, a[idx])));

    return res;
}
