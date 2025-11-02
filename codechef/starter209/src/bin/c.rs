use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use library::{
    cumulative_sum::CumulativeSum,
    utils::{
        chlibs::ChLibs,
        input::Input,
        iterlibs::{collect::CollectIter, dedup::RleItertor},
        veclibs::VecLibs,
    },
};

fn solve(ip: &mut Input) {
    let (n, _) = ip.pair();
    let mut a = ip.vector::<u64>(n);
    let mut b = ip.vector::<u64>(n);
    a.sort();
    b.sort();

    let mut ha = HashMap::new();
    let mut hb = HashMap::new();
    for (i, ai) in a.iter().rle() {
        ha.insert(*ai, i);
    }
    for (i, bi) in b.iter().rle() {
        hb.insert(*bi, i);
    }

    let mut sub = 0;
    for (&x, &ax) in ha.iter() {
        let bx = *hb.get(&x).unwrap_or(&0);
        let cnt = ax + bx;
        if cnt > n {
            let k = cnt - n;
            sub = 2 * k as u64 * x;
            break;
        }
    }
    println!("{}", a.iter().sum::<u64>() + b.iter().sum::<u64>() - sub);
}

fn main() {
    static IS_MULTI_TEST_CASE: bool = true;
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
