use cps::chlibs::ChLibs;
use cps::warshall_floyd::{DefaultWFelm, WarshallFloyd};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, u64); m],
        q: usize,
    }
    
    let inf = 1u64 << 62;
    let g = edges.iter().fold(vec![vec![]; n], |mut g, &(u, v, t)| { g[u].push((v, t)); g[v].push((u, t)); g });
    let wf = WarshallFloyd::new(&g, DefaultWFelm);
    
    for _ in 0..q {
        input! {
            k: usize,
            b: [Usize1; k],
        }
        let mut ans = inf;
        for v in b.iter().copied().permutations(k) {
            let mut dp = vec![vec![inf; 2]; k];
            dp[0][0] = edges[v[0]].2 + wf.get(0, edges[v[0]].1);
            dp[0][1] = edges[v[0]].2 + wf.get(0, edges[v[0]].0);

            for j in 1..k {
                let initial_cost = edges[v[j]].2;
                // f.0 -> t.0 -> t.1
                let val = dp[j-1][0] + initial_cost + wf.get(edges[v[j-1]].0, edges[v[j]].0);
                dp[j][1].chmin(val);
                
                // f.0 -> t.1 -> t.0
                let val = dp[j-1][0] + initial_cost + wf.get(edges[v[j-1]].0, edges[v[j]].1);
                dp[j][0].chmin(val);

                // f.1 -> t.0 -> t.1
                let val = dp[j-1][1] + initial_cost + wf.get(edges[v[j-1]].1, edges[v[j]].0);
                dp[j][1].chmin(val);
                
                // f.0 -> t.1 -> t.0
                let val = dp[j-1][1] + initial_cost + wf.get(edges[v[j-1]].1, edges[v[j]].1);
                dp[j][0].chmin(val);
            }
            ans.chmin((dp[k-1][0] + wf.get(edges[v[k-1]].0, n-1)).min(dp[k-1][1] + wf.get(edges[v[k-1]].1, n-1))); 
        }
        println!("{ans}");
    }
}
