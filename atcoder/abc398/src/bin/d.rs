use std::collections::BTreeSet;

use itertools::Itertools;
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        n: usize,
        r: i64,
        c: i64,
        s: Chars,
    }
    let mut set = BTreeSet::new();
    set.insert((0, 0));
    let mut pos = (0, 0);
    let mut ans = vec![0; n];
    for (idx, &co) in s.iter().enumerate() {
        match co {
            'N' => pos.0 -= 1,
            'S' => pos.0 += 1,
            'W' => pos.1 -= 1,
            'E' => pos.1 += 1,
            _ => unreachable!()
        }
        set.insert((-pos.0, -pos.1));

        let target = (r-pos.0, c-pos.1);
        if set.contains(&target) {
            ans[idx] = 1;
        }
    }
    println!("{}", ans.iter().join(""));
}

