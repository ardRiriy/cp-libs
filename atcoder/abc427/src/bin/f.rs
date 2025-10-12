use std::collections::HashMap;

use library::utils::{input::Input, iterlibs::collect::CollectIter};

fn func(v: &Vec<u32>, m: u32) -> Vec<HashMap<u32, u64>> {
    let mut dp = vec![HashMap::new(); 2];
    dp[0].insert(0, 1u64);
    for &ai in v.iter() {
        let mut ndp = vec![HashMap::new(); 2];

        for (&k, &cnt) in dp[0].iter() {
            let nk = (k + ai) % m;
            // 選ぶ
            *ndp[1].entry(nk).or_insert(0) += cnt;
            // no
            *ndp[0].entry(k).or_insert(0) += cnt;
        }
        for (&k, &cnt) in dp[1].iter() {
            *ndp[0].entry(k).or_insert(0) += cnt;
        }

        dp = ndp;
    }

    return dp;
}

fn solve(ip: &mut Input) {
    let n = ip.next();
    let m = ip.next();
    let a = ip.vector::<u32>(n);

    let prev = a[0..n / 2].iter().copied().collect_vec();
    let pdp = func(&prev, m);

    let rerr = a[n / 2..].iter().rev().copied().collect_vec();
    let rdp = func(&rerr, m);

    let mut ans = 0;

    for i in 0..2 {
        for (&k, &cnt) in pdp[i].iter() {
            let targ = (m - k) % m;
            if let Some(val) = rdp[0].get(&targ) {
                ans += cnt * *val;
            }
            if i == 0 {
                if let Some(val) = rdp[1].get(&targ) {
                    ans += cnt * *val;
                }
            }
        }
    }

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
