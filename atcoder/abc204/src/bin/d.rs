#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    static N: usize = 2e5 as usize;
    let mut dp = vec![false; N];
    dp[0] = true;
    for ai in &a {
        for i in (0..N).rev() {
            if !dp[i] { continue; }
            let ni = i + *ai;
            if ni < N {
                dp[ni] = true;
            }
        }
    }

    let sum = a.iter().sum::<usize>();
    let ans = (0..N)
        .filter(|i| dp[*i] && sum >= *i)
        .map(|i| i.max(sum-i))
        .min()
        .unwrap();
    println!("{ans}");
}

