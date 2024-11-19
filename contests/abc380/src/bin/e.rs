use cps::unionfind::UnionFind;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::iter::repeat;
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let merge = |x: &(usize, usize), y: &(usize, usize)| -> (usize, usize) {
        (x.0.min(y.0), (x.1.max(y.1)))
    };
    let mut uf = UnionFind::new(n, merge);
    let mut color = (0..n).collect_vec();
    let mut ans = repeat(1).take(n).collect_vec();
    for i in 0..n {
        uf.insert_data(i, (i, i));
    }

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            1 => {
                input! {
                    x: Usize1,
                    c: Usize1,
                }
                ans[color[uf.leader(x)]] -= uf.size(x);
                ans[c] += uf.size(x);
                color[uf.leader(x)] = c;
                loop {
                    let left = uf.get_data(x).unwrap().0;
                    if left == 0 || color[uf.leader(left - 1)] != c {
                        break;
                    }
                    uf.merge(x, left - 1);
                }
                loop {
                    let right = uf.get_data(x).unwrap().1;
                    if right == n - 1 || color[uf.leader(right + 1)] != c {
                        break;
                    }
                    uf.merge(x, right + 1);
                }
            }
            2 => {
                input! {
                    c: Usize1,
                }
                println!("{}", ans[c]);
            }
            _ => unreachable!(),
        }
    }
}
