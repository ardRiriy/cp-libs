use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        h: [Usize1; n],
    } 

    let mut ans = 1;
    for i in 0..n {
        for steps in 1.. {
            if i + steps >= n {
                break;
            }
            let mut tmp = 1;
            let mut k = steps;

            while i + k < n {
                if h[i + k] == h[i] {
                    tmp += 1;
                    k += steps;
                } else {
                    break;
                }
            }
            ans.chmax(tmp);
        }
    }

    println!("{}", ans);
}

