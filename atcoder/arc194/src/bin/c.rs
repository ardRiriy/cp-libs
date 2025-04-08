use std::cmp::Reverse;
use std::collections::BinaryHeap;
use ac_library::{Monoid, Segtree};
use itertools::Itertools;
use proconio::input;
use rand::Rng;

fn main() {
    input!{
        n: usize,
        a: [u64; n],
        b: [u64; n],
        c: [u64; n],
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut z = vec![];
    let mut start_cost = 0;
    for i in 0..n {
        if a[i] == 1 {
            start_cost += c[i];
        }
        match (a[i], b[i]) {
            (1, 1) => {
                z.push(c[i]);
            },
            (1, 0) => {
                y.push(c[i]);
            },
            (0, 1) => {
                x.push(c[i]);
            },
            _ => {}
        }
    }

    let mut cur = start_cost;
    let mut ans = 0;
    for &yi in y.iter().sorted_unstable().rev() {
        cur -= yi;
        ans += cur;
    }
    for &xi in x.iter().sorted_unstable().rev() {
        cur += xi;
        ans += cur;
    }


    for zi in z {
        

    }



}


fn gen() {
    let mut n = 7;
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut c = vec![0; n];

    let mut rng = rand::thread_rng();
    for i in 0..n {
        a[i] = rng.gen_range(0..=1);
        b[i] = rng.gen_range(0..=1);
        c[i] = rng.gen_range(0..=100);
    }
    println!("{}", a.iter().join(" "));
    println!("{}", b.iter().join(" "));
    println!("{}", c.iter().join(" "));
}
