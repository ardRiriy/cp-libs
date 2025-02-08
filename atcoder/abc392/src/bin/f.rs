use ac_library::{Monoid, Segtree};
use itertools::Itertools;
use proconio::input;

struct Sum;
impl Monoid for Sum {
    type S = i64;
    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a + b
    }
}

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut seg: Segtree<Sum> = Segtree::new(n);
    for i in 0..n {
        seg.set(i, 1);
    }

    let mut ans = vec![0; n];
    for (i, pi) in p.iter().enumerate().rev() {
        // 初めてsumがpi以上になるところが答え
        // 木上の二分探索
        let idx = seg.max_right(0, |&sum| sum < *pi as i64);
        seg.set(idx, 0);
        ans[idx] = i + 1;
    }
    println!("{}", ans.iter().join(" "));
}
