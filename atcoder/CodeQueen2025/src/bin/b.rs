use std::collections::BTreeMap;

use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let (n, s) = ip.pair::<usize, u64>();
    let a = ip.vector::<u64>(n);
    
    let mut map = BTreeMap::new();
    for ai in a.iter() {
        *map.entry(*ai).or_insert(0) += 1u64;
    }

    let ans = a.iter()
        .map(|ai| {
            if s < *ai { return 0; }
            let t = s-*ai;
            if let Some(k) = map.get(&t) {
                if *ai == t { *k - 1 } else { *k }
            } else {
                0
            }
        })
        .sum::<u64>();
    println!("{}", ans/2);
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
