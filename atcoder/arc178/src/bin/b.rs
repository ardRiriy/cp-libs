use std::mem::swap;

use itertools::Itertools;
use proconio::input;
use ac_library::ModInt998244353 as Mint;

fn solve() -> Mint {
    input! {
        mut a1: u64,
        mut a2: u64,
        a3: u64,
    }
    if a1 > a2 {
        swap(&mut a1, &mut a2);
    }
    if a2 + 1 < a3 {
        return Mint::new(0);
    }

    if a2 == a3 {
        // 繰り上がらない場合の数
        
    } else {
        // 繰り上がる場合
    }

    Mint::new(0)
}

fn main() {
    input! {
        t: usize
    }
    let ans = (0..t).map(|_| solve()).join("\n");
    println!("{}", ans);
}