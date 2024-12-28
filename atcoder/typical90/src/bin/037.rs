use ac_library::{Monoid, Segtree};
use proconio::input;

const MIN_VAL:i64 = -(1 << 60);

struct Max;
impl Monoid for Max {
    type S = i64;

    fn identity() -> Self::S {
        MIN_VAL
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a.max(b)
    }
}

fn main() {
    input! {
        w: usize,
        n: usize,
        item: [(usize, usize, i64); n],
    }

    let mut dp :Segtree<Max> = Segtree::new(w+1);
    dp.set(0, 0);
    for (l, r, v) in item {
        for j in (0..=w).rev() {
            if j < l {
                break;
            }
            let upper = j - l;
            let lower = if j > r { j - r } else { 0 };

            let nxt = dp.prod(lower..=upper);
            if nxt == MIN_VAL {
                continue;
            }
            dp.set(j, (nxt + v).max(dp.get(j))); 
        }
    }

    let ans = dp.get(w);
    println!("{}", if ans == MIN_VAL { -1 } else { ans });
}
