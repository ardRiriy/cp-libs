use std::collections::BTreeMap;

use library::{cumulative_sum::CumulativeSum, utils::{chlibs::ChLibs, input::Input}};

fn solve(ip: &mut Input) {
    let (n, k) = ip.pair::<usize,i64>();
    let a = ip.vector::<i64>(n);
    
    let b = a.iter().map(|ai| *ai - k).collect::<Vec<_>>();
    let cb = CumulativeSum::new(&b);
    
    // S_iに対するiの最大/最小
    let mut min_idx = BTreeMap::new();
    let mut max_idx = BTreeMap::new();
    
    for i in 0..=n {
        let val = cb.get(0..i);
        if let None = min_idx.get_mut(&val) {
            min_idx.insert(val, i);
        }
        
        max_idx.insert(val, i);
    }
    
    let mut cur = 0;
    let mut ans = 0;
    for  (k, i) in min_idx.iter() {
        if let Some(&id) = max_idx.get(k) {
            cur.chmax(id);
        } else {
            panic!();
        }

        if cur > *i {
            ans.chmax(cur-i);
        }
        
    }
    println!("{}", ans);
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
