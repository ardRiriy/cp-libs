use proconio::{input, marker::Usize1};

fn dfs(
    p: usize,
    g: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    visited_order: &mut Vec<usize>,
) {
    seen[p] = true;
    for &ni in g[p].iter() {
        if seen[ni] {
            continue;
        }
        dfs(ni, g, seen, visited_order);
    }
    visited_order.push(p);
}

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(Usize1, Usize1); m],
    }

    let g = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| { g[u].push(v); g });

    let mut visited_order = vec![];
    let mut seen = vec![false; n];

    for i in 0..n {
        if seen[i] {
            continue;
        }
        dfs(i, &g, &mut seen, &mut visited_order);
    }

    let revg = e.iter().fold(vec![vec![]; n], |mut g, &(u, v)| { g[v].push(u); g });
    let mut seen = vec![false; n];
    let mut ans = 0u64;
    for &i in visited_order.iter().rev() {
        if seen[i] {
            continue;
        }
        
        let mut cnt = 0;
        let mut stk = vec![i];
        seen[i] = true;
        while let Some(p) = stk.pop() {
            cnt += 1;
            for &ni in revg[p].iter() {
                if !seen[ni] {
                    seen[ni] = true;
                    stk.push(ni);
                }
            }
        }

        ans += cnt * (cnt - 1) / 2;
    }

    println!("{}", ans);
}
