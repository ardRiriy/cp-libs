#[allow(unused_imports)]
use cps::debug::*;
use ac_library::ModInt998244353 as Mint;
use proconio::{input, marker::Usize1};

fn factor(i: usize, fac: &mut Vec<Mint>) -> Mint{
    if i == 0 {
        fac[i] = Mint::new(1);
        return fac[i];
    }
    let ret = factor(i-1, fac);
    fac[i] = Mint::new(i) * ret;
    return fac[i];
}

fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }
    let friends = e.iter()
        .fold(vec![vec![false; 2*n]; 2*n], |mut v, &(a, b)| {
            v[a][b] = true;
            v[b][a] = true;
            v
        });

    let mut fac = vec![Mint::new(n); n+2];
    factor(n+1, &mut fac);
    
    // dp[i][j] := 生徒iから始めて、j組作成する作成の通り数
    let mut dp = vec![vec![Mint::new(0); n+1]; 2*n+1];
    for i in 0..2*n+1 {
        // 生徒iを右端において、そこから0組作る構成数は1
        dp[i][0] += 1;
    }

    for j in 1..=n {
        for i in 0..2*(n-j)+1 {

            for k in 0..j {
                if friends[i][i+2*k+1] {
                    let val = dp[i+1][k] * dp[i+2*k+2][j-k-1] * fac[j] / (fac[k+1] * fac[j-k-1]); 
                    dp[i][j] += val;
                }
            }
        }
    }
    println!("{}", dp[0][n]);
}

