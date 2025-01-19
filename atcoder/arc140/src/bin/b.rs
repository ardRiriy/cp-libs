#[allow(unused_imports)]
use cps::debug::*;
use cps::multiset::MultiSet;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        s: Chars,
    }

    let mut mset = MultiSet::new();
    let mut entry_count = 0;
    for (i, _) in s.iter().enumerate().filter(|&(_, c)| *c=='R') {
        let mut l = 0;
        for j in (0..i).rev() {
            if s[j] == 'A' {
                l += 1;
            } else {
                break;
            }
        }
        let mut r = 0;
        for j in i+1..n {
            if s[j] == 'C' {
                r += 1;
            } else {
                break;
            }
        }
        let rank = l.min(r);
        if rank != 0 {
            mset.add(rank, 1);
            entry_count += 1;
        }
    }

    let mut ans = 0;
    while entry_count > 0 {
        // 奇数回目の操作
        let (&key, _) = mset.max_key().unwrap();
        mset.remove(key, 1);
        if key > 1 {
            mset.add(key-1, 1);
        } else {
            entry_count -= 1;
        }
        ans += 1;
        if entry_count == 0 { break; }

        // 偶数回目の操作
        let (&key, _) = mset.min_key().unwrap();
        mset.remove(key, 1);
        entry_count -= 1;
        ans += 1;
    }
    println!("{}", ans);
}
