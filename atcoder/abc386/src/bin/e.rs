use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [u64; n],
    }

    if n / 2 <= k {
        let all_xor = a.iter().fold(0, |acc, x| acc ^ x);
        let mut ans = 0;
        for v in a.iter().combinations(n-k) {
            let sub_xor = v.iter().fold(0, |acc, &x| acc ^ x);
            ans.chmax(all_xor ^ sub_xor);
        }
        println!("{}", ans);
    } else {
        let mut ans = 0;
        for v in a.iter().combinations(k) {
            let sub_xor = v.iter().fold(0, |acc, &x| acc ^ x);
            ans.chmax(sub_xor);
        }
        println!("{}", ans);
    }
}


