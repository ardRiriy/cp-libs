use std::{collections::{BTreeMap, BTreeSet}, vec};

use ac_library::{Mod998244353, ModInt998244353, StaticModInt};
use itertools::Itertools;
use proconio::{input};


fn solve() {
    input!{
        n: usize,
        a: [i64; n]
    }

    let mut g :Vec<BTreeMap<i64, Vec<usize>>> = vec![BTreeMap::new(); n];

    for i in 0..n {
        for j in i+1..n {
            let diff = a[j] - a[i];

            if let Some(v) = g[i].get_mut(&diff) {
                v.push(j);
            } else {
                g[i].insert(diff, vec![j]);
            }
        }
    }


    println!("{:?}", g);

    let mut dat = vec![vec![ModInt998244353::new(0); n+1]; n+1];
    for i in 0..n+1 {
        dat[i][0] = ModInt998244353::new(i+1);
        dat[i][i] = ModInt998244353::new(1);
        for j in 1..i {
            dat[i][j] = dat[i-1][j-1] + dat[i-1][j];
        }
    }

    let mut ans = vec![ModInt998244353::new(0); n];
    let mut seen = BTreeSet::new();

    for i in 0..n {
        ans[0] += 1;

        for (&diff, v) in &g[i] {
            if seen.contains(&diff) {
                continue;
            }

            seen.insert(diff);

            let mut stk = vec![];
            for j in v {
                stk.push((1, *j));
            }


            while let Some((dpt, idx)) = stk.pop() {
                eprintln!("idx: {} | dpt: {}", idx+1, dpt);
                eprintln!("before: {}", ans.iter().join(" "));
                for i in 1..=dpt {
                    ans[i] += dat[dpt][i] - dat[dpt-1][i];
                }
                eprintln!("after: {}", ans.iter().join(" "));
                eprintln!();

                if let Some(v) = g[idx].get(&diff) {
                    for j in v {
                        stk.push((dpt+1, *j));
                    }
                }
            }

            eprintln!("idx: {} | diff: {}", i+1, diff);
            eprintln!("{}", ans.iter().join(" ")); 
            eprintln!("-----------------");  
        }

    }
    
    println!("{}", ans.iter().join(" "));
}

/*

            ▄▌▐▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▌
     ▄▄██▌█            宅急便です！
▄▄▄▌▐██▌█ Rating +25 :) をお届けに参りました！
███████▌█▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▌
▀(⊙)▀▀▀▀(⊙)(⊙)▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀(⊙

*/

static INF: u64 = 1e18 as u64;

trait ChLibs<T: std::cmp::Ord> {
    fn chmin(&mut self, elm: T) -> bool;
    fn chmax(&mut self, elm: T) -> bool;
}

impl<T: std::cmp::Ord> ChLibs<T> for T {
    fn chmin(&mut self, elm: T) -> bool {
        return if *self > elm {
            *self = elm;
            true
        } else {
            false
        };
    }

    fn chmax(&mut self, elm: T) -> bool {
        return if *self < elm {
            *self = elm;
            true
        } else {
            false
        };
    }
}

fn main() {
    // input! { mut i: usize }
    let mut i = 1;
    while i != 0 {
        solve(); 
        i -= 1;
    }
}

