#[allow(unused_imports)]
use cps::debug::*;
use cps::mink_sum::MaxK;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        k: usize,
        q: usize,
    }
    let mut maxk = MaxK::new(k);
    let mut a = vec![0; n];
    for &ai in a.iter() {
        maxk.add(ai);
    }

    for _ in 0..q {
        input! {
            x: Usize1,
            y: u64,
        }
        maxk.remove(a[x]);
        a[x] = y;
        maxk.add(a[x]);
        println!("{}", maxk.ans());
    }
}

