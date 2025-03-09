use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        q: usize,
    }

    let mut su = (0..n).collect_vec();
    let mut p = (0..n).collect_vec();
    let mut hato = (0..n).collect_vec();
    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                a: Usize1,
                b: Usize1,
            }
            let b = su[b];
            hato[a] = b;
        } else if t == 2 {
            input! {
                a: Usize1,
                b: Usize1,
            }
            let ai = su[a];
            let bi = su[b];
            p.swap(ai, bi);
            su[p[ai]] = ai;
            su[p[bi]] = bi;
        } else {
            input! {
                a: Usize1,
            }
            println!("{}", p[hato[a]] + 1);
        }
    }
}
