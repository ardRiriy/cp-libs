#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        n: usize,
        s: u64,
        mut a: [u64; n],
    }

    let s = s % a.iter().sum::<u64>();

    // aを2回繰り返した配列を作る
    for i in 0..n {
        a.push(a[i]);
    }

    let mut sum = 0;
    let mut r = 0;
    let mut l = 0;
    while r < a.len() {
        if sum == s {
            println!("Yes");
            return;
        }

        if sum < s {
            sum += a[r];
            r += 1;
        } else {
            sum -= a[l];
            l += 1;
        }
    }
    
    while l < a.len() {
        if sum == s {
            println!("Yes");
            return;
        }
        sum -= a[l];
        l += 1;
    }
    println!("No");
}

