use std::collections::HashMap;

use library::utils::{input::Input, iterlibs::dedup::RleItertor};

fn solve(ip: &mut Input) {
    let n = ip.next();
    let mut a = ip.vector::<u64>(n);
    a.sort();
    let mut map = HashMap::new();
    for (cnt, ai) in a.iter().rle() {
        map.insert(*ai, cnt);
    }

    let ans = map
        .iter()
        .map(|(_, &cnt)| cnt * (cnt - 1) * (n - cnt))
        .sum::<usize>();
    println!("{}", ans / 2);
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
