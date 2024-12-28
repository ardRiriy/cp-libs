use cps::chlibs::ChLibs;
use cps::consts::INF;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input!{
        n: usize,
        mg: usize,
        emg: [(Usize1, Usize1); mg],
        mh: usize,
        emh: [(Usize1, Usize1); mh],
    }

    let cost = (1..n).rev()
        .map(|i| {
            input! {
                a: [u64; i],
            }
            a
        })
        .collect_vec();

    let g = emg.iter()
        .fold(vec![vec![]; n], |mut g, (u, v)| {
            g[*u].push(*v);
            g[*v].push(*u);
            g
        });

    let h = emh.iter()
        .fold(vec![vec![]; n], |mut g, (u, v)| {
            g[*u].push(*v);
            g[*v].push(*u);
            g
        });

    let mut ans = INF;

    for v in (0..n).permutations(n) {
        let mut sum = 0;
        for i in 0..n {
            for j in 0..n {
                if i == j || v[i] == v[j] {
                    continue;
                }
                if g[i].contains(&j) != h[v[i]].contains(&v[j]) {
                    let (i, j) = if v[i] < v[j] { (v[i], v[j]) } else { (v[j], v[i]) };

                    sum += cost[i][j-i-1];
                }
            }
        }
        ans.chmin(sum / 2);
    }
    println!("{}", ans);
}
