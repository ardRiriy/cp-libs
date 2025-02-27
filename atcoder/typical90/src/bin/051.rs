use cps::consts::INF;
use cps::veclibs::VecLibs;
use itertools::Itertools;
use proconio::input;

fn bitdp(a: &Vec<u64>) -> Vec<Vec<u64>> {
    /* aからi個選んだときの値段としてありうるものをres[i]に持つ */
    let mut res = vec![vec![]; a.len()+1];
    let n = a.len();

    let mut dp = vec![INF; 1<<n];
    dp[0] = 0;
    res[0].push(0);
    for i in 0..1<<n {
        assert_ne!(dp[i], INF);
        for j in 0..n {
            let nxt = i | (1<<j);
            if dp[nxt] != INF { continue; } // 到達済み
            dp[nxt] = dp[i] + a[j];
            res[nxt.count_ones() as usize].push(dp[nxt]);
        }
    }

    res
}

fn main() {
    input!{
        n: usize,
        k: usize,
        p: u64,
        a: [u64; n],
    }
    
    let a_half = a.iter().copied().take(n/2).collect_vec();
    let b_half = a.iter().copied().skip(n/2).collect_vec();

    let a_res = bitdp(&a_half);
    let mut b_res = bitdp(&b_half);
    let mut ans = 0;

    for i in 0..=k {
        // a_res[i]の各要素について
        if !(i < a_res.len() && k-i < b_res.len()) { continue; }
        
        b_res[k-i].sort_unstable();
        for &ai in a_res[i].iter() {
            if p < ai { continue; }
            let idx = b_res[k-i].lower_bound(p-ai+1);
            ans += idx;
        }
    }
    println!("{}", ans);

}