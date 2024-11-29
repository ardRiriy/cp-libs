use std::collections::VecDeque;

#[allow(unused_imports)]
use cps::debug::*;
use proconio::input;

fn main() {
    input!{
        q: usize,
    }
    
    let mut que = VecDeque::new();
    
    for _ in 0..q {
        input! {
            t: u8
        }
        
        if t == 1 {
            input! {
                x: u64,
                count: usize,
            }
            que.push_back((x, count));
        } else {
            input! {
                mut c: usize,
            }
            let mut ans = 0;
            loop {
                let (x, count) = que.pop_front().unwrap();
                if c < count {
                    let left = count - c;
                    ans += x * c as u64;
                    que.push_front((x, left)); 
                    break;
                } else {
                    ans += x * count as u64;
                    c -= count;
                    if c == 0 {
                        break;
                    }
                }
            }
            println!("{ans}");
        }
    }
}

