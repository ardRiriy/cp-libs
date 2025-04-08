use ac_library::{Min, Monoid, Segtree};
use proconio::{input, marker::{Chars, Usize1}};

struct Sum;
impl Monoid for Sum {
    type S=i64;

    fn identity() -> Self::S {
        0
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        a+b
    }
}

fn main() {
    input!{
        n: usize,
        q: usize,
        mut s: Chars,
    }

    let mut min_seg :Segtree<Min<i64>> = Segtree::new(n);
    let mut sum_seg :Segtree<Sum> = Segtree::new(n);

    for (i, c) in s.iter().enumerate() {
        if *c == '(' {
            min_seg.set(i, 1);
            sum_seg.set(i, 1);
        } else {
            min_seg.set(i, -1);
            sum_seg.set(i, -1);
        }
    }

    for _ in 0..q {
        input! {
            t: u8,
            l: Usize1,
            r: Usize1,
        }
        if t == 1 {
            let cl = s[l];
            let cr = s[r];
            if cl == '(' {
                min_seg.set(r, 1);
                sum_seg.set(r, 1);
            } else {
                min_seg.set(r, -1);
                sum_seg.set(r, -1);
            }

            if cr == '(' {
                min_seg.set(l, 1);
                sum_seg.set(l, 1);
            } else {
                min_seg.set(l, -1);
                sum_seg.set(l, -1);
            }
            s.swap(l, r);

        } else {
            if min_seg.prod(l..=r) >= 0 && sum_seg.prod(l..=r) == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

