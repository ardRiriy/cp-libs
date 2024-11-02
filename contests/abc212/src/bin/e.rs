use ac_library::{Mod998244353, ModInt998244353 as Mint, StaticModInt};
use proconio::{input, marker::Usize1};
/*
素直なDPを考えるとdp[i][j] := i番目の街としてjに到達するような通り数 となるが， 空間NK，遷移NでO(N^2 K)となり間に合わない．
通行止めになっている道はたかだかM=5000程度なので，これを使うことを考える．
dp[i+1][j] = Σ{l=1 -> N} if (iとlをつなぐ道が存在する) dp[i][l] else 0
           = (Σ{l=1 -> N} dp[i][l]) - (Σ{l=1 -> N} if (iとlをつなぐ道が存在する) 0 else dp[i][l])

これで，遷移が各回O(N+M)でできるので，全体でO(K(N+M))となり十分高速．

*/

fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
        mut e: [(Usize1, Usize1); m],
    }
    let mut g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| { g[u].push(v); g[v].push(u); g });
    for i in 0..n {
        g[i].push(i);
    }
    let mut dp = vec![vec![Mint::new(0); n]; 2];
    let mut cur = 0;

    dp[0][0] = Mint::new(1);

    for _ in 0..k {
        let nxt = 1 - cur;
        let sum = dp[cur]
            .iter()
            .sum::<StaticModInt<Mod998244353>>();
        for j in 0..n {
            let sub = g[j].iter()
                .map(|p| dp[cur][*p])
                .sum::<StaticModInt<Mod998244353>>();
            dp[nxt][j] = sum - sub;
        }
        dp[cur].fill(Mint::new(0));
        cur = nxt;

    }
    println!("{}", dp[cur][0]);
}
