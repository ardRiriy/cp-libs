use cps::chlibs::ChLibs;
use cps::warshall_floyd::{DefaultWFelm, WarshallFloyd};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(Usize1, Usize1, i64); m],
    }

    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u,v, w)| {
            g[u].push((v, w));
            g
        });

    let wf = WarshallFloyd::new(&g, DefaultWFelm);

    static INF :i64 = 1<<60;
    let mut dp = vec![vec![INF; n]; 1<<n];
    for i in 0..n {
        dp[1<<i][i] = 0;
    }

    for i in 0..1<<n {
        for j in 0..n {
            if dp[i][j] == INF {
                continue;
            }
            
            for k in 0..n {
                if wf.get(j, k) == i64::max_value() {
                    continue;
                }
                let val = dp[i][j] + wf.get(j, k);
                dp[i|(1<<k)][k].chmin(val);
            }
        }
    }

    if dp[(1<<n)-1].iter().all(|vi| vi == &INF) {
        println!("No");
    } else {
        println!("{}", dp[(1<<n)-1].iter().min().unwrap());
    }
}