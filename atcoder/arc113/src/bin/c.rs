#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn to_i(c: char) -> usize {
    (c as usize) - 'a' as usize
}

fn main() {
    input!{
        s: Chars,
    }

    let mut addr = vec![0; 26];
    let mut sum = vec![0; 26];

    let rle = s.iter().copied().dedup_with_count().collect_vec();
    let mut last = (b'z' + 1) as char;

    for (count, c) in rle {
        for (i, val) in addr.iter().enumerate() {
            if to_i(c) == i && last == c {
                if *val != 0 {
                    sum[i] += count * (val-1); 
                }
            } else {
                sum[i] += count * *val;
            }
        }

        if last != c && count >= 2 {
            last = c;
            addr[to_i(c)] += 1;
        }
    }

    println!("{}", sum.iter().sum::<usize>());
}

/*
..aabbaaca...
というときに最後のaは直前のaaからは更新されないが
最初のaaからは更新ができる
*/
