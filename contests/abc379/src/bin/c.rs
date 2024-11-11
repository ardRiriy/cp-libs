
use std::{collections::BinaryHeap, process::exit};

use itertools::Itertools;
use proconio::input;
fn main() {
    input!{
        n: usize,
        m: usize,
        mut x: [usize; m],
        mut a: [usize; m],
    }
    
    let mut x = x.iter()
        .enumerate()
        .map(|i| (*i.1, i.0))
        .sorted()
        .collect_vec();
    
    if x[0].0 != 1 {
        println!("-1");
        return;
    }
    x.push((n+1, n+1));
    a.push(0);
    
    fn sum(n: usize) -> usize {
        n * (n + 1) / 2
    }
    let mut amari = BinaryHeap::new();

    let mut ans = 0;
    for idx in 0..m {
        let i = x[idx].0;
        let num = a[x[idx].1];
        let j = x[idx + 1].0;
        let require = j - i;
        if require <= num {
            let left = num - require;
            if left != 0 {
                amari.push((i, left));
            }
            ans += sum(require - 1);
        } else {
            let mut num = num;
            let mut flag = false;
            while let Some((j, add)) = amari.pop() {
                if num + add >= require {
                    let left = require - num;

                    ans += (i - j) * left;
                    if add != left {
                        amari.push((j, add - left));
                    }
                    flag = true;
                    break;
                } else {
                    num += add;
                    ans += (i - j) * add;
                }
            }
            if flag {
                ans += sum(require - 1);
            } else {
                println!("-1");
                exit(0);
            }
        } 
    } 

    if amari.is_empty() {
        println!("{ans}");
    }else {
        println!("-1");
    }
}

