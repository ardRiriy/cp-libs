use std::collections::VecDeque;

use library::utils::{input::Input, iterlibs::collect::CollectIter};

fn solve(ip: &mut Input) {
    let n :usize = ip.next();
    let k :u64 = ip.next();

    let p = ip.vector::<usize>(n-1);
    let a = ip.vector::<u64>(n-1);

    let mut g = vec![vec![]; n];
    for i in 0..n-1 {
        g[p[i]-1].push((i+1, a[i]));
    }

    let mut dp = vec![vec![None; 2]; n];
    dp[0][0] = Some(k);
    dp[0][1] = Some(k);
    
    let mut que = VecDeque::new();
    que.push_back(0);

    while let Some(p) = que.pop_front() {
        let unf = dp[p][0].unwrap();
        let fix = dp[p][1].unwrap();

        for &(ni, ai) in g[p].iter() {
            dp[ni][0] = Some(unf - unf / ai);    
            dp[ni][1] = Some((fix - fix / ai).max(unf));
            que.push_back(ni);
        }
    }

    let mut unfs = dp.iter().map(|v| v[0].unwrap()).collect_vec();
    unfs.sort();
    
    let ans = (0..n).map(|i| {
        if dp[i][0].unwrap() == unfs[0] {
            unfs[1].min(dp[i][1].unwrap())
        } else {
            unfs[0].min(dp[i][1].unwrap())
        }
    }).max().unwrap();
    println!("{}", ans);

}

fn main() {
    static IS_MULTI_TEST_CASE :bool = true;
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
