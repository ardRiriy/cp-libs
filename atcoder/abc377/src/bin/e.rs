use cps::modpow::modpow;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

/*
実験をすると、i -> p_iに辺を張ってできるサイクルの上を
p_iから始めて1回, 2回, 4回, ...と進んでいることがわかる。

これをk回行えばΣ_(i=0)^{K-1} 2^i = 2^k - 1回進むこととなる(iから見ると2^k回進んでいる)
サイクルの検出と作成をするのがO(N), 各頂点で行き先を調べる操作が繰り返し２乗法を使って全体でO(Nlog(K))なので、全体でO(Nlog(K))で求めることができる。

注: サイクルの数ごとで計算をすれば答えを求めるフェーズの計算量はサイクル数をMとしてO(Mlog(K)) (→ 公式解説)だが、
    サンプル2のような場合ではNlog(K)になるのであんまり気にする必要はない
*/

fn main() {
    input! {
        n: usize,
        k: u64,
        p: [Usize1; n],
    }
    let inf = 1 << 62;
    let mut pos = vec![(inf, inf); n];
    let mut cycles = vec![];
    for i in 0..n {
        if pos[i].0 != inf {
            continue;
        }

        let mut v = vec![];
        let mut current = i;
        loop {
            v.push(current);
            pos[current] = (cycles.len(), v.len() - 1);
            current = p[current];
            if current == i {
                break;
            }
        }
        cycles.push(v);
    }

    let mut ans = vec![];
    for i in 0..n {
        let m = cycles[pos[i].0].len();
        let steps = modpow(2, k as u64, m as u64) as usize;
        ans.push(cycles[pos[i].0][(pos[i].1 + steps) % m]);
    }
    println!("{}", ans.iter().map(|x| *x + 1).join(" "));
}
