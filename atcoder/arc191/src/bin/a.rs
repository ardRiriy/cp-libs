use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque}};

use cps::chlibs::ChLibs;
#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!{
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }
    let mut sv = s.iter().map(|c| c.to_digit(10).unwrap()).collect_vec();

    let mut v = vec![vec![]; 10];
    for (i, ti) in t.iter().enumerate() {
        let ti = ti.to_digit(10).unwrap();
        v[ti as usize].push(i);
    }

    let mut last_used = -1; // これ以降のものはどこかで消費しないといけない
    let mut p = vec![BinaryHeap::new(); 10];
    for i in 0..n {
        for j in (sv[i] as usize+1..10).rev() {
            if v[j].is_empty() { continue; }

            let idx = v[j].pop().unwrap();
            sv[i] = j as u32;
            last_used.chmax(idx as i32);
            break;
        }
        if i+1 != n {
            p[sv[i] as usize].push(Reverse(i));
        }
    }

    for i in (last_used+1) as usize..m  {
        let ti = t[i].to_digit(10).unwrap() as usize;
        let mut min_idx = -1;
        for j in 1..ti {
            if let Some(Reverse(idx)) = p[j].pop() {
                p[j].push(Reverse(i));
                min_idx.chmax(idx as i32);
            }
        }
        if min_idx != -1 {
            p[sv[min_idx as usize] as usize].pop();
            sv[min_idx as usize] = ti as u32;
            p[sv[min_idx as usize] as usize].push(Reverse(min_idx as usize));
        } else {
            if p[ti as usize].is_empty() {
                sv[n-1] = ti as u32;
            }
        }
    }


    println!("{}", sv.iter().join(""));
}
