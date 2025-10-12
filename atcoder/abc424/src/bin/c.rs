use std::collections::VecDeque;

use library::utils::{input::Input, iterlibs::collect::CollectIter};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let v = (0..n).map(|_| ip.pair::<usize>()).collect_vec();
    
    let mut seen = vec![false; n];
    let mut que = VecDeque::new();

    let g = v.iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &(u,v))| {
            if u == 0 {
                seen[i] = true;
                que.push_back(i);
                return acc;
            }
            acc[u-1].push(i);
            acc[v-1].push(i);
                
            acc
    });

    while let Some(p) = que.pop_front() {
        for ni in g[p].iter() {
            if seen[*ni] {
                continue;
            }
            seen[*ni] = true;
            que.push_back(*ni);
        }
    }

    println!("{}", seen.iter().filter(|&b| *b).count());
}

fn main() {
    static IS_MULTI_TEST_CASE :bool = false;
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
