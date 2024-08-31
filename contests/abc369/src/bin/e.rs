use std::{cmp::Reverse, collections::BinaryHeap, io::{stdout, BufWriter}};
use std::io::Write;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

// (800 * log_2(800))^2 = 59521225
// 800^3                = 512000000
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

    let distances = (0..n)
        .map(|x| dijkstra(&g, x))
        .collect_vec();

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

fn dijkstra(g: &Vec<Vec<(usize, u64)>>, start: usize) -> Vec<u64> {
    let n = g.len();
    let mut dist = vec![INF; n];
    let mut left = n;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, pos))) = heap.pop() {
        if dist[pos] != INF {
            continue;
        }
        dist[pos] = cost;
        left -= 1;
        if left == 0 {
            break;
        }

        for &(ni, w) in &g[pos] {
            if dist[ni] == INF {
                heap.push(Reverse((cost + w, ni)));
            }
        }
    }
    dist
}
pub trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        return if *self > elm {
            *self = elm;
            true
        } else {
            false
        };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return if *self < elm {
            *self = elm;
            true
        } else {
            false
        };
    }
}
pub static INF: u64 = 1e18 as u64;
pub static DI: &[usize] = &[0, !0, 0, 1];
pub static DJ: &[usize] = &[!0, 0, 1, 0];
pub static DC: &[char] = &['L', 'U', 'R', 'D'];
