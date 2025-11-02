use std::collections::VecDeque;

use library::utils::{
    input::Input,
    iterlibs::{collect::CollectIter, strs::StrUtilIter},
};

fn dfs(
    p: usize,
    seen: &mut Vec<bool>,
    g: &Vec<Vec<usize>>,
    res: &mut Vec<Vec<(u32, usize, usize)>>,
) -> (u32, usize) {
    seen[p] = true;
    for &ni in g[p].iter() {
        if seen[ni] {
            continue;
        }
        let (t, f) = dfs(ni, seen, g, res);
        res[p].push((t, f, ni));
    }
    res[p].sort();
    res[p].reverse();

    if res[p].is_empty() {
        (1, p)
    } else {
        (res[p][0].0 + 1, res[p][0].1)
    }
}

fn dfs2(
    p: usize,
    parent: (u32, usize, usize),
    g: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    res: &mut Vec<Vec<(u32, usize, usize)>>,
    ans: &mut Vec<usize>,
) {
    seen[p] = true;

    let mut top = vec![parent];
    for i in 0..2.min(res[p].len()) {
        top.push(res[p][i]);
    }
    top.sort();
    top.reverse();

    ans[p] = top[0].1;

    for &ni in g[p].iter() {
        if seen[ni] {
            continue;
        }
        let mut nxt = if top[0].2 == ni { top[1] } else { top[0] };
        nxt.0 += 1;
        dfs2(ni, nxt, g, seen, res, ans);
    }
}

fn solve(ip: &mut Input) {
    let n = ip.next();
    let g = ip.graph(n, n - 1, false);

    let mut seen = vec![false; n];
    let mut res = vec![vec![]; n];
    let mut ans = vec![0; n];
    dfs(0, &mut seen, &g, &mut res);
    seen.fill(false);
    dfs2(0, (0, 0, 0), &g, &mut seen, &mut res, &mut ans);
    println!("{}", ans.iter().map(|i| i + 1).join("\n"))
}

fn solve2(ip: &mut Input) {
    let n = ip.next();
    let g = ip.graph(n, n - 1, false);

    let bfs = |s: usize| -> Vec<usize> {
        let mut res = vec![None; n];
        let mut que = VecDeque::new();
        que.push_back(s);
        res[s] = Some(0);
        while let Some(p) = que.pop_front() {
            for &ni in g[p].iter() {
                if res[ni].is_some() {
                    continue;
                }
                res[ni] = Some(res[p].unwrap() + 1);
                que.push_back(ni);
            }
        }
        res.iter().copied().flatten().collect_vec()
    };

    let res1 = bfs(0);
    let s = {
        let d = *res1.iter().max().unwrap();
        res1.iter()
            .enumerate()
            .filter(|(_, di)| di == &&d)
            .last()
            .unwrap()
            .0
    };
    let res2 = bfs(s);
    let t = {
        let d = *res2.iter().max().unwrap();
        res2.iter()
            .enumerate()
            .filter(|(_, di)| di == &&d)
            .last()
            .unwrap()
            .0
    };

    let res3 = bfs(t);
    println!(
        "{}",
        (0..n)
            .map(|i| {
                if res2[i] == res3[i] {
                    s.max(t) + 1
                } else if res2[i] > res3[i] {
                    s + 1
                } else {
                    t + 1
                }
            })
            .join("\n")
    );
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
        solve2(&mut ip);
    }
}
