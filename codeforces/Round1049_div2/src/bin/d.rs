use std::{cmp::Reverse, collections::BTreeSet};

use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next();
    let v = (0..n).map(|_|ip.pair::<i64>()).collect::<Vec<_>>();
    
    let mut v1 = v.iter().enumerate().map(|(i, v)| (*v, i)).collect::<Vec<_>>();
    v1.sort_by_key(|b| b.0);

    let mut v2 = v.iter().enumerate().map(|(i, v)| (*v, i)).collect::<Vec<_>>();
    v2.sort_by_key(|val| Reverse(val.1));

    let mut used = BTreeSet::new();
    
    let mut ans = 0;
    for (val, i) in v1[..n/2].iter() {
        used.insert(i);
        ans -= val.0;
    }
    
    let mut cnt = 0;
    for (val, i) in v2.iter() {
        if used.contains(&i) { continue; }
        ans += val.1;
        cnt += 1;
        if cnt == n/2 {
            break;
        }
    }
    dbg!(ans);
    println!("{}", v.iter().map(|(l,r)| r-l).sum::<i64>() + ans);

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
