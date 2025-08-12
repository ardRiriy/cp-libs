use std::collections::BTreeMap;

use library::utils::input::Input;

fn solve(ip: &mut Input) {
    let n = ip.next::<usize>();
    let a = ip.vector::<usize>(n);

    let mut map = BTreeMap::new();
    for (i, &ai) in a.iter().enumerate() {
        *map.entry(i + 1 + ai).or_insert(0) += 1u64;
    }

    let ans = a
        .iter()
        .enumerate()
        .map(|(i, ai)| {
            if *ai > i + 1 {
                0
            } else if let Some(c) = map.get(&(i + 1 - ai)) {
                *c
            } else {
                0
            }
        })
        .sum::<u64>();
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
