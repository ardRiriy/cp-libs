use ac_library::{Max, Segtree};
#[allow(unused_imports)]
use cps::debug::*;
use cps::consts::INF;
use cps::veclibs::VecLibs;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        mut a: [u64; n],
        q: usize,
    }
    a.push(INF);
    let mut seg :Segtree<Max<usize>> = Segtree::new(n);
    for (i, ai) in a[0..n].iter().enumerate() {
        seg.set(i, a.lower_bound(*ai * 2) - i);
    }

    let check = |l: usize, r: usize, k: usize| -> bool {
        let val = seg.prod(l..l+k);
        l+k-1+val.max(k)<=r
    };

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1
        }

        // 無駄にlogがつくが、シンプルな二分探索をする
        let mut ok = 0;
        let mut ng = n-l;

        while ng-ok>1 {
            let mid = (ok+ng)/2;
            if check(l, r, mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{}", ok);
    }
}

