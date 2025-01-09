use ac_library::{FenwickTree, ModInt998244353 as Mint};
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    }

    static M: usize = 5000; // A_iとしてあり得る最大値

    let indicate = (0..n).sorted_unstable_by_key(|i| a[*i]).collect_vec();
    let mut dp = FenwickTree::new(M+1, Mint::new(0));
    dp.add(0, 1);
    let mut ans = Mint::new(0);
    for i in indicate {
        if a[i] >= b[i] {
            let key = a[i] - b[i];
            ans += dp.sum(..=key as usize);
        }

        for j in (0..M).rev() {
            if j+b[i] as usize > M {
                continue;
            }
            let adder = dp.sum(j..=j);
            dp.add(j+b[i] as usize, adder);
        }
    }
    println!("{}", ans);
}
