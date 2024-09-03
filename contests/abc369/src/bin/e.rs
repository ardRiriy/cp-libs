use std::io::{stdout, BufWriter};
use std::io::Write;
use cps::graph::warshall_floyd;
use cps::consts::INF;
use cps::chlibs::ChLibs;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, u64); m],
        q: usize,
    }
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let g = edges.iter()
        .fold(
        vec![vec![]; n],
            |mut acc, &(u, v, k)| {
                acc[u].push((v, k));
                acc[v].push((u, k));
                acc
            }
        );

    let distances = warshall_floyd(&g);

    for _ in 0..q {
        input! {
            k: usize,
            lists: [Usize1; k]
        }

        let ans = lists.iter()
            .map(|idx| edges[*idx].2)
            .sum::<u64>();
        let mut min = INF;
        for lists in lists.iter().copied().permutations(k) {
            let lists = [
                vec![vec![0, 0]],
                lists.iter()
                    .map(|idx| vec![edges[*idx].0, edges[*idx].1])
                    .collect_vec(),
                vec![vec![n-1, n-1]]
            ].concat();
            let k = lists.len();

            let mut dp = vec![vec![INF; 2]; k];
            dp[0][0] = 0;

            for i in 0..k-1 {
                for j in 0..2 {
                    if dp[i][j] == INF {
                        continue;
                    }
                    let from = lists[i][1-j];
                    for l in 0..2 {
                        let to = lists[i+1][l];
                        let dist = distances[from][to];
                        let adder = dp[i][j];
                        dp[i+1][l].chmin(dist + adder);
                    }
                }
            }
            min.chmin(dp[k-1][0].min(dp[k-1][1]));
        }
        writeln!(out, "{}", ans + min).unwrap();
    }
}
