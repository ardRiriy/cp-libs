use std::collections::BTreeSet;

use library::utils::{input::Input, iterlibs::collect::CollectIter};

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair();
    let s = (0..n).map(|_| ip.chars()).collect_vec();
    let mut set = BTreeSet::new();
    for i in 0..n {
        'j: for j in 0..n {
            let mut v = vec![];
            for di in 0..m {
                for dj in 0..m {
                    let pi = i + di;
                    let pj = j + dj;
                    if pi >= n || pj >= n {
                        continue 'j;
                    }
                    v.push(s[pi][pj]);
                }
            }
            set.insert(v);
        }
    }
    println!("{}", set.len());
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
