use ac_library::FenwickTree;
use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        n: usize,
        s: Chars
    }
    
    let mut bit = FenwickTree::new(n, 0);
    for (i, &c) in s.iter().enumerate() {
        let val = (c as u8 - b'0') as u64 * (i as u64 + 1);
        bit.add(i, val);
    }
    
    let mut ans = vec![];
    let mut left = 0;
    
    for i in (0..n).rev() {
        let sum = bit.sum(0..=i) + left;

        ans.push(sum % 10);
        left = sum / 10;
    }
    if left == 0 {
        println!("{}", ans.iter().rev().join(""));
    } else {
        println!("{}{}", left, ans.iter().rev().join(""));
    }
    
}

