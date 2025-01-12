#[allow(unused_imports)]
use cps::debug::*;
use cps::veclibs::VecLibs;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [u64; n]
    }

    let mut ans = 0;
    for i in 0..n {
        let k = a[i]*2;
        let idx = a.lower_bound(k);
        ans += n-idx;
    }
    println!("{}", ans);
}

