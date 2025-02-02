#[allow(unused_imports)]
use cps::debug::*;
use itertools::iproduct;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }
    'ol: for (i, j) in iproduct!(0..n, 0..n) {
        for (di, dj) in iproduct!(0..m, 0..m) {
            let ni = i+di;
            let nj = j+dj;
            if ni >= n || nj >= n {
                continue 'ol;
            }
            if s[ni][nj] != t[di][dj] {
                continue 'ol;
            }
        }
        println!("{} {}", i+1, j+1);
        break;
    }
}

