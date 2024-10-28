use cps::modpow::modpow;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: u64,
        p: [Usize1; n],
    }
    let inf = 1 << 62;
    let mut pos = vec![(inf, inf); n];
    let mut cycles = vec![];
    for i in 0..n {
        if pos[i].0 != inf {
            continue;
        }

        let mut v = vec![];
        let mut current = i;
        loop {
            v.push(current);
            pos[current] = (cycles.len(), v.len() - 1);
            current = p[current];
            if current == i {
                break;
            }
        }
        cycles.push(v);
    }

    let mut ans = vec![];
    for i in 0..n {
        let m = cycles[pos[i].0].len();
        let steps = modpow(2, k as u64, m as u64) as usize;
        ans.push(cycles[pos[i].0][(pos[i].1 + steps) % m]);
    }
    println!("{}", ans.iter().map(|x| *x + 1).join(" "));
}
