use cps::chlibs::ChLibs;
use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input!{
        _n: usize,
        s: Chars,
    }

    let v = s.iter()
        .dedup_by_with_count(|a, b| a == b)
        .collect_vec();
    let mut ans = 0;

    if v.len() >= 2 {
        for i in 0..v.len() - 2 {
            if *v[i].1 == '1' && *v[i+2].1 == '2' && *v[i+1].1 == '/' && v[i+1].0 == 1 {
                ans.chmax(v[i].0.min(v[i+2].0) * 2 + 1);
            }
        }
    }

    if ans == 0 && s.contains(&'/') {
        ans = 1;
    }
    println!("{}", ans);
}

