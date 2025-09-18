use std::iter::repeat;

use library::utils::{consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let (n, m) = ip.pair();
    let w = ip.vector::<u64>(n);
    let g = ip.graph(n, m, false);

    let mut dist = vec![vec![INF; n]; n];
    dist[0] = repeat(0).take(n).collect();
    for k in (1..n).rev() {
        for p in 0..n {
            if dist[p][k] == INF { continue; }
            let cost = w[p] * k as u64;
            for &ni in g[p].iter() {
                dist[ni][k-1] = dist[ni][k-1].min(dist[p][k] + cost);
            }
        }
    }
    
    println!("{}", dist.iter().map(|v| v[0].to_string()).collect::<Vec<_>>().join("\n"));
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
