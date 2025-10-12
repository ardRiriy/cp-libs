use std::collections::{BTreeSet, VecDeque};

use library::utils::{
    consts::{DI, DJ},
    input::Input,
    iterlibs::collect::CollectIter,
};

fn solve(ip: &mut Input) {
    let (h, w) = ip.pair();
    let s = (0..h).map(|_| ip.chars()).collect_vec();

    let takahashi = s
        .iter()
        .enumerate()
        .find_map(|(i, r)| {
            r.iter()
                .enumerate()
                .find(|(_, c)| c == &&'T')
                .map(|(j, _)| (i, j))
        })
        .unwrap();

    let state = s
        .iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.iter()
                .enumerate()
                .filter_map(move |(j, c)| if c == &'#' { Some((i, j)) } else { None })
        })
        .collect::<BTreeSet<_>>();

    let op = |s: &BTreeSet<(usize, usize)>, r: usize| -> Option<BTreeSet<(usize, usize)>> {
        let mut checker = false;
        let res = s
            .iter()
            .filter_map(|(pi, pj)| {
                let ni = pi.wrapping_add(DI[r]);
                let nj = pj.wrapping_add(DJ[r]);
                if ni >= h || nj >= w {
                    return None;
                }
                if (ni, nj) == takahashi {
                    checker = true;
                }
                Some((ni, nj))
            })
            .collect();

        if checker {
            None
        } else {
            Some(res)
        }
    };

    let mut seen = BTreeSet::new();
    let mut que = VecDeque::new();
    que.push_back((state, 0));
    while let Some((state, dist)) = que.pop_front() {
        if state.is_empty() {
            println!("{}", dist);
            return;
        }

        for r in 0..4 {
            if let Some(nxt) = op(&state, r) {
                if seen.insert(nxt.clone()) {
                    que.push_back((nxt, dist + 1));
                }
            }
        }
    }
    println!("-1");
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
