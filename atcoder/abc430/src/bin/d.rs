use std::collections::BTreeSet;

use library::utils::{chlibs::ChLibs, input::Input};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let x = ip.vector::<u64>(n);

    let mut d = vec![None; n + 1];

    let mut set = BTreeSet::new();
    set.insert((0u64, 0usize));

    let mut ans = 0u64;

    for (i, &xi) in x.iter().enumerate() {
        let i = i + 1;

        let mut dist = 1 << 60;

        // left
        if let Some(&(xj, j)) = set.range(..(xi, i)).next_back() {
            let diff = xi - xj;
            dist.chmin(diff);
            if let Some(di) = d[j].as_mut() {
                if *di > diff {
                    ans -= *di;
                    ans += diff;
                    *di = diff;
                }
            } else {
                d[j] = Some(diff);
                ans += diff;
            }
        }

        if let Some(&(xj, j)) = set.range((xi, i)..).next() {
            let diff = xj - xi;
            dist.chmin(diff);
            if let Some(di) = d[j].as_mut() {
                if *di > diff {
                    ans -= *di;
                    ans += diff;
                    *di = diff;
                }
            } else {
                d[j] = Some(diff);
                ans += diff;
            }
        }

        d[i] = Some(dist);
        ans += dist;
        set.insert((xi, i));
        println!("{}", ans);
    }
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
