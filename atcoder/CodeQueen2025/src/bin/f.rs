use std::{cmp::Reverse, collections::{BTreeSet, BinaryHeap}};

use library::utils::{consts::INF, input::Input};

fn solve(ip: &mut Input) {
    let (n, m, k) = ip.triple::<usize, usize, usize>();
    let a = ip.vector::<usize>(k).iter().map(|i| *i-1).collect::<Vec<_>>();
    let g = ip.weighted_graph::<u64>(n, m, false, true);
    
    let livehouse = BTreeSet::from_iter(a.iter().copied());


    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0, 0, !0));

    // seen[i][j][k] := 
    let mut seen = vec![vec![vec![(INF, !0); 2]; 2]; n];
    while let Some((Reverse(c), p, q, adr)) = que.pop() {

    }

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
