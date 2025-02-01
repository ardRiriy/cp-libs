use std::collections::VecDeque;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
        w: [u64; n],
        a: [u64; n],
    }

    let mut in_e = vec![0; n];
    let g = e.iter()
        .fold(vec![vec![]; n], |mut g, &(u, v)| {
            if w[u] > w[v] {
                g[v].push(u);
                in_e[u] += 1;
            } else if w[u] < w[v] {
                g[u].push(v);
                in_e[v] += 1;
            }
            g
        });

    let mut v = vec![0; n];
    let mut op :Vec<Vec<(usize, u64)>> = vec![vec![]; n];
    let mut que = VecDeque::from_iter(in_e.iter().enumerate().filter_map(|(i, &vi)| if vi == 0 { Some(i) } else { None }));
    let mut dp;
    while let Some(p) = que.pop_front() {

        dp = vec![0; w[p] as usize];
        dp[0] = 1;
        for &(i, wi) in op[p].iter() {
            for j in (0..w[p] as usize).rev() {
                if j as u64 + w[i] >= w[p] {
                    continue;
                }
                dp[j+w[i] as usize] = dp[j+w[i] as usize].max(dp[j] + wi);
            }
        }
        v[p] = *dp.iter().max().unwrap();
        for &ni in g[p].iter() {
            op[ni].push((p, v[p]));
            in_e[ni] -= 1;
            if in_e[ni] == 0 {
                que.push_back(ni);
            }
        }
    }

    let ans = a.iter().enumerate().map(|(i, ai)| *ai * v[i]).sum::<u64>();
    println!("{}", ans);

}

