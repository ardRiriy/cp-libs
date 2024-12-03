use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

const INF :i64 = 1 << 60;

struct SumMax;
impl Monoid for SumMax {
    type S = i64;

    fn identity() -> Self::S {
        -INF
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a + *b
    }
}

struct AffineMax;
impl MapMonoid for AffineMax {
    type M = SumMax;
    type F = i64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        f + x
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f + g
    }
}


fn main() {
    input!{
        n: usize,
        d: i64,
        point: [(i64, i64); n]
    }
    const MAX :usize = 3e6 as usize + 1;
    const BASE :i64 = 1e6 as i64;

    let mut seg1 :LazySegtree<AffineMax> = LazySegtree::new(MAX);
    let mut seg2 :LazySegtree<AffineMax> = LazySegtree::new(MAX);

    for &(x, y) in point.iter() {
        let i = (x + BASE) as usize;
        dbg!(i);
        if y >= 0 {
            let val = seg1.get(i);
            if val == -INF {
                seg1.set(i, i as i64 + y);
            } else {
                seg1.set(i, val + i as i64 + y);
            }
        } else {
            let val = seg2.get(i);
            if val == -INF {
                seg2.set(i, i as i64 - y);
            } else {
                seg2.set(i, val + i as i64 - y);
            }
        }
    }

    let mut ans = 0;

    for i in 0..MAX {
        // 下にどれぐらい伸ばせるか？
        let val = seg1.all_prod();
        if val >= 0 && d >= val {
            ans += d - val;
            dbg!(i, ans, val);
        } 

        // 上にどれぐらい伸ばせるか
        let val = d - seg2.all_prod();
        if val >= 0 && d >= val {
            ans += d - val;
            dbg!(i, ans, val);
        }

        seg1.apply_range(0..=i, 1);
        seg1.apply_range(i.., -1);

        seg2.apply_range(0..=i, 1);
        seg2.apply_range(i.., -1);
    }
    println!("{ans}")
}
