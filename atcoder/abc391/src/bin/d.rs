use std::cmp::Reverse;
use std::collections::BinaryHeap;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        w: usize,
        v: [(Usize1, Usize1); n],
        q: usize,
        queries: [(usize, Usize1); q],
    }

    let mut g = vec![BinaryHeap::new(); w];
    for (i, &(x, y)) in v.iter().enumerate() {
        g[x].push(Reverse((y, i)));
    }

    let mut kieru_jikan = vec![None; n];
    let mut base_time = 0;
    'lp: loop {
        let mut mh = 0;
        let mut list = vec![];
        for i in 0..w {
            if let Some(Reverse((y, p))) = g[i].pop() {
                list.push(p);
                if base_time > y {
                    continue;
                }
                mh = mh.max(y - base_time);
            } else {
                break 'lp;
            }
        }
        let t = base_time + mh+1;
        for i in list {
            kieru_jikan[i] = Some(t);
        }
        base_time = t;
    }
    let ans = queries.iter().map(|(t, a)| if let Some(tt) = kieru_jikan[*a] {
            if tt <= *t {
                "No"
            } else {
                "Yes"
            }
        } else {
            "Yes"
        }
    ).join("\n");
    println!("{ans}");

}

