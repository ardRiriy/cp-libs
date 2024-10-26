use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: u64,
        p: [Usize1; n],
    }

    let mut doubling = vec![vec![0; n]; 62];
    for i in 0..n {
        doubling[0][i] = p[i]; 
    }

    for j in 0..61 {
        for i in 0..n {
            doubling[j + 1][i] = doubling[j][doubling[j][i]];
        }
    }
    let mut ans = p;

    for i in 0..62 {
        if (k >> i) & 1 == 1 {
            let mut next = vec![0; n];
            for j in 0..n {
                next[j] = doubling[i][ans[j]];
            }
            ans = next;
        }
    }

    dbg!(&doubling[1]);
    println!("{}", ans.iter().map(|&x| x + 1).join(" "));
}
