use ac_library::SccGraph;
use ac_library::ModInt998244353;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

type Mint = ModInt998244353;

fn rec(i: usize, dp: &mut Vec<Vec<Option<Mint>>>, g: &Vec<Vec<usize>> ,j: usize) -> Mint {
    // 計算済みならdp[i][j]の値を返す
    if let Some(res) = dp[i][j] {
        return res;
    }

    let mut res = Mint::new(0);
    if j > 0 {
        if let Some(v) = dp[i][j-1] {
            res += v;
        } else {
            res += rec(i,dp,g,j-1);
        }
    }

    let mut tmp = Mint::new(1);
    for nidx in 0..g[i].len() {
        tmp *= rec(g[i][nidx], dp, g, j);
    }

    dp[i][j] = Some(res + tmp);
    res + tmp
}

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [Usize1; n],
    }

    // a[i] -> iのグラフとしてみる
    let mut c = SccGraph::new(n);

    for (i, ai) in a.iter().enumerate() {
        c.add_edge(*ai, i);
    }

    // 縮約
    let scc = c.scc();

    // 頂点iは縮約後の頂点par[i]とする
    let mut par = vec![0;n];

    for v in scc {
        let p = v[0];
        for i in v {
            par[i] = p;
        }
    }

    let mut s = vec![];
    let mut g = vec![vec![]; n];

    for (i, ai) in a.iter().enumerate() {
        if par[*ai] == par[i] {
            s.push(par[*ai]);
        } else {
            g[par[*ai]].push(par[i]);
        }
    }

    s = s.iter().sorted().unique().copied().collect_vec();

    // dp[i][j] := 頂点iをj以下の値にするときのあり得る組み合わせの数 mod 998
    let mut dp = vec![vec![None; m]; n+1];

    let ans = s.iter()
        .map(|&si| rec(si,&mut dp, &g,m-1))
        .fold(Mint::new(1), |acc, v| acc*v);
    println!("{}", ans);
}

