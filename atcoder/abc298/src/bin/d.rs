use std::collections::VecDeque;

#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;
use ac_library::ModInt998244353 as Mint;

fn main() {
    input!{
        q: usize,
    }
    let mut num = Mint::new(1);
    let mut que = VecDeque::new();
    que.push_back(1);

    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                x: u64,
            }
            num *= 10;
            num += x;
            que.push_back(x);
        } else if t == 2 {
            let t = que.len();
            let x = que.pop_front().unwrap();
            num -= Mint::new(x) * Mint::new(10).pow(TryInto::<u64>::try_into(t).unwrap()-1);
        } else {
            println!("{}", num);
        }
    }
}

