use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    }
    static N: usize = 2e5 as usize + 2;
    let mut cnt_a = vec![0; N];
    let mut cnt_b = vec![0; N];
    for i in 0..n {
        cnt_a[a[i] as usize] += 1;
        cnt_b[b[i] as usize] += 1;
    }

    if (0..N).any(|i| cnt_a[i] + cnt_b[i] > n) {
        println!("No");
        return;
    }

    let mut ai = 0;
    let mut bi = 0;
    let mut d = 0;

    while ai < n && bi < n {
        if a[ai] == b[bi] {
            d = d.max(ai.abs_diff(bi));
            ai += 1;
            bi += 1;
        } else if a[ai] < b[bi] {
            ai += 1;
        } else {
            bi += 1;
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[(i+d)%n] = b[i];
    }

    println!("Yes");
    println!("{}", ans.iter().join(" "));
}