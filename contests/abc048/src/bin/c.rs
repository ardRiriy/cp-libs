#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        x: u64,
        mut a: [u64; n],
    }

    let mut ans = 0;
    if a[0] > x {
        ans += a[0] - x;
        a[0] = x;
    }

    for i in 0..n-1 {
        let s = a[i] + a[i+1];
        if s > x {
            let over = s - x;
            ans += over;
            a[i+1] = if a[i+1] > over { a[i+1] - over } else { 0 };
        }
    }
    println!("{ans}");
}

