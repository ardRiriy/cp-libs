use std::collections::BTreeSet;

use library::utils::{consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair();
    let mut edges = Vec::new();

    for _ in 0..m {
        let (u, v) = ip.pair::<usize>();
        edges.push((u-1,v-1));
    }

    let ans = (0..1<<n)
        .map(|target| {
            edges
                .iter()
                .filter(|&(u,v)| 
                    ((target >> u) & 1) == ((target >> v) & 1)
                )
                .count()
        })
        .min()
        .unwrap();

    

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
