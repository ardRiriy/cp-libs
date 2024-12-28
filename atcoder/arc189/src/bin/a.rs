use ac_library::ModInt998244353 as Mint;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;
/*
はじめの状態をSと書いたとき、
A_i != A_{i+1}でS_i != A_i あるいは S_{i+1} != A_{i+1}のとき構築不可
=> 1 0 1 0 1みたいな奇数の数単位だけ考えればいい

長さ2*N+1の奇数の列 → ランクN
1回操作するとランクN-1
=> ランクN-1であり得る操作回数 * ランクNの初手の選び方(= 2*N-1, ランク1なら1)

↑をf(r) = ランクrの操作回数 として
ブロックがK個合ってブロックiがランクR_iのとき

並べ方？
*/

fn op_count(rank: usize, memo: &mut Vec<Mint>) -> Mint {
    if rank == 1 {
        memo[rank] = Mint::new(1);        
        return memo[rank];
    }

    let mut res = Mint::new(2) * rank - 1;
    let ret = op_count(rank-1, memo);
    res *= ret;
    memo[rank] = res;
    res
}

fn factor(n: usize, memo: &mut Vec<Mint>) -> Mint {
    if n == 1 {
        memo[n] = Mint::new(1);
        return memo[n];
    } 
    let res = factor(n-1, memo) * n;
    memo[n] = res;
    return res;
}
static N: usize = 1e5 as usize + 3;

fn main() {
    input!{
        n: usize,
        a: [i32; n],
    }
    let mut f = vec![Mint::new(0); N+1];
    op_count(N, &mut f);

    let mut factors = vec![Mint::new(0); N+1];
    factor(N, &mut factors);

    let mut v = vec![];
    let mut l = 0;
    for (i, &ai) in a[0..n-1].iter().enumerate() {
        if ai != a[i+1] {
            let r = i + 1 - l;
            if r % 2 == 0 || (a[l] as usize == l % 2 && a[i] as usize == i % 2) {
                // 構築不可
                println!("0");
                return;
            } else {
                if (r-1)/2 != 0 {
                    v.push((r-1)/2);
                }
                l = i+1;
            }
        }
    }
    {
        let r = n - l;
        if r % 2 == 0 {
            println!("0");
            return;
        } else {
            if (r-1)/2 != 0 {
                v.push((r-1)/2);
            }
        }
    }
    // すでに一致している
    if v.is_empty() {
        println!("0");
        return;
    }

    // 選び方
    let mut choose_way = Mint::new(1);
    for &rank in &v {
        choose_way *= f[rank];
    }
    
    let vs = v.iter().map(|rank| *rank).sum::<usize>();
    let mut sita = Mint::new(1);
    for &ai in v.iter() {
        sita *= factors[ai];
    }
    println!("{}", choose_way * factors[vs] / sita);
}
