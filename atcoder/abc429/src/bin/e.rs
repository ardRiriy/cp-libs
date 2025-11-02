use std::collections::VecDeque;

use library::utils::{input::Input, iterlibs::strs::StrUtilIter};

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair();
    let g = ip.graph(n, m, false);
    let s = ip.chars();

    let mut seen = vec![vec![None; 10]; n];
    let mut que = VecDeque::new();
    for (i, c) in s.iter().enumerate() {
        if c == &'S' {
            seen[i][0] = Some((0u64, i));
            que.push_back((i, 0));
        }
    }
    while let Some((p, fi)) = que.pop_front() {
        let (pd, psi) = seen[p][fi].unwrap();
        for &ni in g[p].iter() {
            for j in 0..10 {
                if let Some((_, si)) = seen[ni][j] {
                    if si == psi {
                        break;
                    }
                } else {
                    seen[ni][j] = Some((pd + 1, psi));
                    que.push_back((ni, j));
                    break;
                }
            }
        }
    }
    let ans = s
        .iter()
        .enumerate()
        .filter(|(_, c)| c == &&'D')
        .map(|(i, _)| seen[i][0].unwrap().0 + seen[i][1].unwrap().0)
        .join("\n");
    println!("{}", ans);
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = false;
    let mut ip = Input::new();

    let t = if IS_MULTI_TEST_CASE {
        ip.next::<usize>()
    } else {
        1
    };

    for _ in 0..t {
        solve(&mut ip);
    }
}
