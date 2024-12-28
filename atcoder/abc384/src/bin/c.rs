use std::cmp::Reverse;

#[allow(unused_imports)]
use cps::debug::*;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        v: [u64; 5],
    }

    let sv = ["A", "B", "C", "D", "E"];

    let mut ans = vec![];

    for i in 1..1<<5 {
        let mut sum = 0;
        let mut name = String::new();
        for j in 0..5 {
            if i >> j & 1 == 1 {
                sum += v[j];
                name.push_str(sv[j]);
            }
        }
        ans.push((sum, name));
    }

    println!("{}", ans.iter().sorted_by_key(|(s, n)| (Reverse(s), n)).map(|(_, name)| name).join("\n"));

}

