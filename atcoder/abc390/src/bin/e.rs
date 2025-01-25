use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn dp(key: i64, x: usize, v: &Vec<(i64, usize)>) -> usize {
    let mut dp = vec![-1; x+1];
    dp[0] = 0;
    for &(a, c) in v {
        for i in (0..x+1).rev() {
            if i+c >= x+1 {
                continue;
            }
            let val = dp[i] + a;
            dp[i+c].chmax(val);
        }
    }
    // key以上で最小のindex
    for (i, &xi) in dp.iter().enumerate() {
        if xi >= key {
            return i;
        }
    }
    return x+1;
}

fn main() {
    input!{
        n: usize,
        x: usize,
        v: [(Usize1, i64, usize); n],
    }
 
    let v = v.iter().fold(vec![vec![]; 3], |mut v, &(i, a, c)| {
        v[i].push((a, c));
        v
    });


    let mut ok = 0;
    let mut ng = 1e18 as i64;
    // let mut ng = 10;
    while ng-ok>1 {
        let mid = (ok+ng)/2;
        let s = (0..3).into_iter().map(|i| {
            let res = dp(mid, x, &v[i]);
            res
        }).sum::<usize>();

        if s <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

