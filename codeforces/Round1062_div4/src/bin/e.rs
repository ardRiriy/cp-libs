use std::collections::BTreeSet;

use library::{
    algorithm::binary_search::binary_search,
    utils::{input::Input, iterlibs::strs::StrUtilIter},
};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let (k, x) = ip.pair();
    let mut a = ip.vector::<u64>(n);
    a.sort();

    let ck = |d: u64| -> bool {
        let mut l = 0;
        let mut left = k;
        for (i, r) in a.iter().enumerate() {
            let ld = if i == 0 { l } else { l + d };

            if ld >= *r || *r < d {
                l = *r;
                continue;
            }
            let rd = *r - d;
            if ld > rd {
                l = *r;
                continue;
            }
            let d = rd + 1 - ld;
            left = if left <= d { 0 } else { left - d };
            l = *r;
        }
        let ld = a[n - 1] + d;
        if ld <= x {
            let d = x + 1 - ld;
            //dbg!(d);
            left = if left <= d { 0 } else { left - d };
        }

        left == 0
    };

    // /dbg!(ck(1000000000));

    let d = binary_search(0, 3e9 as u64, &ck);

    let mut l = 0;
    let mut ans = BTreeSet::new();
    'ans: for (i, r) in a.iter().enumerate() {
        let ld = if i == 0 { l } else { l + d };

        if ld >= *r || *r < d {
            l = *r;
            continue;
        }
        let rd = *r - d;
        if ld > rd {
            l = *r;
            continue;
        }
        for p in ld..=rd {
            ans.insert(p);
            if ans.len() == k as usize {
                break 'ans;
            }
        }
        l = *r;
    }
    for p in a[n - 1] + d.. {
        if ans.len() == k as usize {
            break;
        }
        ans.insert(p);
    }
    println!("{}", ans.iter().join(" "));
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = true;
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
