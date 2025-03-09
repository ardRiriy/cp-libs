use cps::chlibs::ChLibs;
use cps::consts::INF;
#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        s: Chars,
    }
    let mut l = 0u64;
    let mut r = 0;


    let mut cur = 0;
    {
        let mut z_cnt = 0;
        for i in 0..n {
            if s[i] == '0' {
                z_cnt += 1;
            } else {
                cur += z_cnt;
                r += 1;
            }
        }
    }

    let mut ans = INF;
    for i in 0..n {
        if s[i] == '0' {
            cur += l;
            cur -= r;
        } else if s[i] == '1' {
            r -= 1;
            ans.chmin(cur);
            l += 1;
        }
    }
    println!("{}", ans);
}

