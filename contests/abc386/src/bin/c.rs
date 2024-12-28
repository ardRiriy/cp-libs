#[allow(unused_imports)]
use cps::debug::*;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        _k: usize,
        s: Chars,
        t: Chars,
    }

    if s.len() == t.len() {
        let mut st = false;
        for i in 0..s.len() {
            if s[i] != t[i] {
                if st {
                    println!("No");
                    return;
                }
                st = true;
            }
        }
    } else if s.len() + 1 == t.len() {
        let mut st = false;
        let mut si = 0;
        let mut ti = 0;
        while si < s.len() {
            if s[si] != t[ti] {
                if st {
                    println!("No");
                    return;
                }
                st = true;
                ti += 1;
            } else {
                si += 1;
                ti += 1;
            }
        }
    } else if s.len() == t.len() + 1 {
        let mut st = false;
        let mut si = 0;
        let mut ti = 0;
        while ti < t.len() {
            if s[si] != t[ti] {
                if st {
                    println!("No");
                    return;
                }
                st = true;
                si += 1;
            } else {
                si += 1;
                ti += 1;
            }
        }
    } else {
        println!("No");
        return;
    }

    println!("Yes");

}

