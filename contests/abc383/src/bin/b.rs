use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use cps::debug::*;
use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        h: usize,
        w: usize,
        d: usize,
        s: [Chars; h],
    }

    let mut ans = 0;
    for (ai, aj) in iproduct!(0..h, 0..w) {
        if s[ai][aj] == '#' {
            continue;
        }
        for (bi, bj) in iproduct!(0..h, 0..w) {
            if ai == bi && aj == bj {
                continue;
            }
            if s[bi][bj] == '#' {
                continue;
            }

            let mut cnt = 0;
            for (i, j) in iproduct!(0..h, 0..w) {
                if s[i][j] == '#' {
                    continue;
                }
                let dist = (ai.abs_diff(i) + aj.abs_diff(j)).min(bi.abs_diff(i) + bj.abs_diff(j));
                if dist <= d {
                    cnt += 1;
                }
            }
            ans.chmax(cnt);
        }
    }
    println!("{ans}");

}

