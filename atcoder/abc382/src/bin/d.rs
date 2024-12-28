use std::collections::BTreeSet;

#[allow(unused_imports)]
use cps::debug::*;
#[allow(unused_imports)]
use cps::lg;
use itertools::Itertools;
use proconio::input;

fn dfs(v: &mut Vec<u64>, n: usize, m: u64, ans: &mut BTreeSet<Vec<u64>>) {
    if v.len() == n {
        ans.insert(v.clone());
        return;
    }
    
    let min = if let Some(e) = v.last() {
        *e + 10
    } else {
        1
    };
    let max = m - (n-v.len()-1) as u64 * 10;

    for x in min..=max {
        v.push(x);
        dfs(v, n, m, ans);
        v.pop();
    }
}

fn main() {
    input!{
        n: usize,
        m: u64,
    }

    let mut ans = BTreeSet::new();
    dfs(&mut vec![], n, m, &mut ans);
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|v| v.iter().join(" ")).join("\n"));
}

