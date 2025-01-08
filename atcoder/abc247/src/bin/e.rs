use ac_library::{Max, Segtree};
#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        x: i64,
        y: i64,
        a: [i64; n],
    }

    let mut min_seg :Segtree<Max<i64>> = Segtree::new(n);
    let mut max_seg :Segtree<Max<i64>> = Segtree::new(n);
    for (i, &ai) in a.iter().enumerate() {
        min_seg.set(i, -ai); // 単調増加になるように負の値をセットしておく
        max_seg.set(i, ai);
    }

    let mut ans = 0;
    for i in 0..n {
        let min_r = min_seg.max_right(i, |ai| *ai <= -y);
        let max_r = max_seg.max_right(i, |ai| *ai <= x);
        let min_l = min_seg.max_right(i, |ai| *ai < -y);
        let max_l = max_seg.max_right(i, |ai| *ai < x);
        if min_r < max_l || max_r < min_l {
            continue;
        }
        ans += (min_r).min(max_r) - min_l.max(max_l);
    }    
    println!("{ans}");


}

