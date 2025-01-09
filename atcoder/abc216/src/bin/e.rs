#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: u64,
        mut a: [u64; n],
    }

    a.push(0); // 番兵
    fn sum(n: u64) -> u64 {
        n * (n+1) / 2
    }

    let a = a.iter()
        .sorted()
        .rev()
        .copied()
        .collect_vec();
    let mut ans = 0;

    for i in 0..n {
        let diff = a[i] - a[i+1];

        let cnt = (i as u64 +1)*diff; // 乗車回数
        if cnt <= k {
            ans += (sum(a[i]) - sum(a[i+1])) * (i as u64 + 1);
            k -= cnt;
        } else {
            let consume = k / (i as u64 + 1);
            let left = k % (i as u64 + 1);

            ans += (sum(a[i]) - sum(a[i]-consume)) * (i as u64 + 1);
            ans += (a[i] - consume) * left;
            break;
        }
    }
    println!("{}", ans);

}
