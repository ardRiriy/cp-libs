use std::{collections::BTreeSet, process::exit};
use itertools::Itertools;
use proconio::input;

/*
sの並べ方を順列前列挙で、
間に入れる_の数をメモ化再帰で全探索する
*/

fn dfs(v: &Vec<String>, set: &BTreeSet<String>, cnt: &mut Vec<usize>, length: usize, memo: &mut BTreeSet<Vec<usize>>) {
    if length > 16 || memo.contains(cnt) {
        return;
    }
    memo.insert(cnt.clone());

    let mut s = v[0].clone();
    for i in 1..v.len() {
        s = format!("{}{}{}", s, (0..cnt[i-1]).map(|_| "_").join(""), v[i]);
    }
    if s.len() >= 3 && !set.contains(&s) {
        println!("{s}");
        exit(0);
    }

    for i in 0..cnt.len() {
        cnt[i] += 1;
        dfs(v, set, cnt, length+1, memo);
        cnt[i] -= 1;
    }
}

fn main() {
    input!{
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }
    let mut set = BTreeSet::new();
    for ti in t {
        set.insert(ti);
    }

    let s_len_sum = s.iter()
        .map(|s| s.len())
        .sum::<usize>();
    let mut cnt = vec![1; n-1];
    for v in s.iter().cloned().permutations(n) {
        dfs(&v, &set, &mut cnt, s_len_sum + n - 1, &mut BTreeSet::new());
    }
    println!("-1");
}

