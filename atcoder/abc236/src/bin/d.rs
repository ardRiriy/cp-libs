#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn dfs(i: usize, seen: &mut Vec<bool>, a: &Vec<Vec<u32>>, cur: u32) -> u32{
    if i == seen.len() {
        return cur;
    }
    if seen[i] {
        return dfs(i+1, seen, a, cur);
    }
    let mut res = 0;
    seen[i] = true;
    for j in i+1..a.len() {
        if seen[j] {
            continue;
        }
        seen[j] = true;
        res = res.max(dfs(i+1, seen, a, cur ^ a[i][j]));
        seen[j] = false;
    }
    seen[i] = false;
    res
} 

fn main() {
    input!{
        n: usize,
    }
    let mut a = vec![vec![0; 2*n]; 2*n];
    for i in 0..2*n-1 {
        for j in i+1..2*n {
            input! {
                x: u32,
            }
            a[i][j] = x;
            a[j][i] = x;
        }
    }
    println!("{}", dfs(0, &mut vec![false; 2*n], &a, 0));
}

