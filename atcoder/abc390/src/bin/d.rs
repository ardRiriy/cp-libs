use std::collections::BTreeSet;

#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn dfs(v: &mut Vec<u64>, i: usize, a: &Vec<u64>, ans: &mut BTreeSet<u64>) {
    if i == a.len() {
        ans.insert(v.iter().fold(0, |acc, &x| acc ^ x));
        return;
    }
    
    for j in 0..v.len() {
        v[j] += a[i]; 
        dfs(v, i+1, a, ans);
        v[j] -= a[i]; 
    }
    v.push(a[i]);
    dfs(v, i+1, a, ans);
    v.pop();
}

fn main() {
    input!{
        n: usize,
        a: [u64; n],
    }

    let mut ans = BTreeSet::new();
    dfs(&mut Vec::new(), 0, &a, &mut ans);
    println!("{}", ans.len());

}


