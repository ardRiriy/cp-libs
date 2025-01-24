#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        k: u64,
        a: [u64; n],
    }

    let judge = |l: u64| -> bool {
        // 長さlで分割したときの操作回数の総和
        a.iter().map(|x| *x / l - if *x % l == 0 { 1 } else { 0 }).sum::<u64>() <= k
    };

    let mut ok = a.iter().copied().max().unwrap();
    let mut ng = 0;
    while ok-ng>1 {
        let m = (ok+ng)/2;
        if judge(m) {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}

